use crate::{
    core::{self, language::dl},
    provider::text_document::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use tokio::sync::Mutex;

// Document symbol provider definitions for ".dl" files.
pub async fn document_symbol(
    tree: &tree_sitter::Tree,
    params: lsp::DocumentSymbolParams,
    content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let result = crate::provider::common::document_symbol::dl::document_symbol(tree, params, content).await?;
    let result = lsp::DocumentSymbolResponse::Flat(result);
    let result = Some(result);
    Ok(result)
}
