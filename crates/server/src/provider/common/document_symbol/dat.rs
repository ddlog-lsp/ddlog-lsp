use crate::core;

pub async fn document_symbol(
    _session: &core::Session,
    _params: lsp::DocumentSymbolParams,
    _content: &ropey::Rope,
) -> anyhow::Result<Vec<lsp::SymbolInformation>> {
    Ok(vec![])
}
