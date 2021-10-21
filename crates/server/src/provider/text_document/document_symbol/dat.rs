use std::sync::Arc;

pub async fn document_symbol(
    _session: Arc<crate::core::Session>,
    _params: lsp::DocumentSymbolParams,
    _content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    Ok(None)
}
