#![allow(unused)]

use lsp_text::RopeExt;
use std::sync::Arc;

mod dat;
mod dl;

/// Encodes data for constructing upcoming DocumentSymbols.
#[derive(Clone, Debug)]
pub struct Data<'tree> {
    /// The tree-sitter Node to be processed as a symbol.
    pub node: tree_sitter::Node<'tree>,
    /// Number of (possibly filtered) children to be processed for the symbol.
    pub children_count: usize,
    /// The kind of document entity the symbol represents.
    pub kind: lsp::SymbolKind,
    /// The name hint for the symbol (used for anonymous entities).
    pub name_hint: &'static str,
}

/// Encodes actions for loop iterations when processing tree-sitter nodes.
#[derive(Debug)]
pub enum Work<'tree> {
    /// Construct a DocumentSymbol and pop the data stack.
    Data,
    /// Add a tree-sitter node to remaining nodes to process.
    Node(tree_sitter::Node<'tree>),
}

/// Convenience type for packaging a (symbol) name with an lsp range and selection range.
#[derive(Clone, Debug)]
pub struct SymbolRange {
    /// The name (identifier) of the symbol.
    pub name: String,
    /// The (node-enclosing) range of the symbol.
    pub range: lsp::Range,
    /// The (identifier-enclosing) range of the symbol.
    pub selection_range: lsp::Range,
}

/// Compute the name and ranges for a document symbol given tree-sitter node data.
pub fn symbol_range(
    content: &ropey::Rope,
    node: tree_sitter::Node,
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

/// Compute "textDocument/documentSymbols" for a given document.
pub async fn document_symbol(
    session: Arc<crate::core::Session>,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let text = session.get_text(&params.text_document.uri).await?;
    let content = text.content.clone().await.ok_or_else(|| {
        anyhow::anyhow!(
            "could not resolve text content for uri: {:#?}",
            params.text_document.uri
        )
    })?;
    let session = session.clone();
    let response = match text.language {
        crate::core::Language::DDlogDat => self::dat::document_symbol(session, params, &content).await?,
        crate::core::Language::DDlogDl => self::dl::document_symbol(session, params, &content).await?,
    };
    Ok(response)
}
