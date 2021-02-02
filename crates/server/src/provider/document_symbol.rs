#![allow(unused)]

use crate::core::{self, RopeExt};
use std::sync::Arc;

mod dat;
mod dl;

#[derive(Clone, Debug)]
pub(self) struct Data<'tree> {
    pub node: &'tree tree_sitter::Node<'tree>,
    pub children_count: usize,
    pub kind: lsp::SymbolKind,
    pub name_hint: &'static str,
}

#[derive(Debug)]
pub(self) enum Work<'tree> {
    Data,
    Node(tree_sitter::Node<'tree>),
}

#[derive(Clone, Debug)]
pub(self) struct SymbolRange {
    pub name: String,
    pub range: lsp::Range,
    pub selection_range: lsp::Range,
}

pub(self) fn symbol_range<'tree>(
    content: &ropey::Rope,
    node: &tree_sitter::Node<'tree>,
    name_hint: &'static str,
    field_id: u16,
) -> SymbolRange {
    let name;
    let range = content.tree_sitter_range_to_lsp_range(node.range());
    let selection_range;
    if let Some(inner_node) = node.child_by_field_id(field_id) {
        name = content.utf8_text_for_tree_sitter_node(&inner_node).into();
        selection_range = content.tree_sitter_range_to_lsp_range(inner_node.range());
    } else {
        name = format!("<{}@{}:{}>", name_hint, range.start.line + 1, range.start.character + 1);
        selection_range = range;
    }

    SymbolRange {
        name,
        range,
        selection_range,
    }
}

pub async fn document_symbol(
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let text = session.get_text(&params.text_document.uri).await?;
    let session = session.clone();
    let response = match text.language {
        core::Language::DDlogDat => self::dat::document_symbol(session, params, &text.content).await?,
        core::Language::DDlogDl => self::dl::document_symbol(session, params, &text.content).await?,
    };
    Ok(response)
}
