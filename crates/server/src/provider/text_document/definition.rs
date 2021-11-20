use crate::core::language::dl;
use lsp_text::RopeExt;
use std::{borrow::Borrow, sync::Arc};

// FIXME: split functionality for {.dl, .dat}

pub fn is_definition_candidate<'tree>(node: &tree_sitter::Node<'tree>) -> bool {
    [dl::kind::IDENT_LOWER_SCOPED, dl::kind::IDENT_UPPER_SCOPED].contains(&node.kind_id())
}

pub async fn definition(
    session: Arc<crate::core::Session>,
    params: lsp::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp::GotoDefinitionResponse>> {
    use crate::analysis::imports;

    let origin_module_uri = &params.text_document_position_params.text_document.uri;

    let text = session.get_text(origin_module_uri).await?;
    let content = text.get_content().await?;

    let tree = session
        .get_tree(origin_module_uri)
        .await?
        .clone()
        .await
        .ok_or_else(|| {
            anyhow::anyhow!(
                "could not open tree for uri: {:#?}",
                params.text_document_position_params.text_document.uri
            )
        })?;
    let tree = tree.lock().await;

    let import_uris = {
        let base = origin_module_uri.clone();
        let mut uris = imports::collect_imports(&content, &tree)
            .map(imports::resolve_import(base))
            .map(|import| import.uri)
            .collect::<Vec<_>>();
        uris.push(origin_module_uri.clone());
        uris
    };

    let mut aggregate_symbols = vec![];
    for uri in import_uris {
        if let Some(symbols) = session.document_symbols.get(&uri) {
            if let Some(symbols) = symbols.value().clone().await {
                let symbols: &Vec<lsp::SymbolInformation> = symbols.borrow();
                for info in symbols {
                    aggregate_symbols.push(info.clone());
                }
            }
        }
    }

    let node = {
        let position = content.lsp_position_to_core(params.text_document_position_params.position)?;
        let start = position.byte;
        let end = position.byte;
        tree.root_node().named_descendant_for_byte_range(start, end)
    };

    if let Some(node) = node {
        if is_definition_candidate(&node) {
            let node_text = content
                .utf8_text_for_tree_sitter_node(&node)
                .split("::")
                .last()
                .expect("valid scoped paths should always have a final component")
                .to_string();

            if let Some(candidate) = aggregate_symbols.iter().find(|info| info.name == node_text) {
                let link = {
                    let origin_selection_range = Some(content.tree_sitter_range_to_lsp_range(node.range()));
                    let target_uri = candidate.location.uri.clone();
                    let target_range = candidate.location.range;
                    let target_selection_range = candidate.location.range;
                    lsp::LocationLink {
                        origin_selection_range,
                        target_uri,
                        target_range,
                        target_selection_range,
                    }
                };

                return Ok(Some(lsp::GotoDefinitionResponse::Link(vec![link])));
            }
        }
    }

    Ok(None)
}
