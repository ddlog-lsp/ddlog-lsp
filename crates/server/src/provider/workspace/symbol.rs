use std::sync::Arc;

pub async fn symbol(
    session: Arc<core::Session>,
    params: lsp::WorkspaceSymbolParams,
) -> anyhow::Result<Option<Vec<lsp::SymbolInformation>>> {
    let session = session.as_ref();
    let result = crate::analysis::document_symbol::dl::document_symbol(session, params, content).await?;
    let result = lsp::DocumentSymbolResponse::Flat(result);
    let result = Some(result);
    Ok(result)
}
