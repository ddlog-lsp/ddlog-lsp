use crate::{
    core::{self, language::dl},
    provider::text_document::document_symbol::{symbol_range, Data, SymbolRange, Work},
};
use std::sync::Arc;

// Document symbol provider definitions for ".dl" files.
pub async fn document_symbol(
    session: Arc<core::Session>,
    params: lsp::DocumentSymbolParams,
    content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    let session = session.as_ref();
    let result = crate::provider::common::document_symbol::dl::document_symbol(session, params, content).await?;
    let result = lsp::DocumentSymbolResponse::Flat(result);
    let result = Some(result);
    Ok(result)
}
