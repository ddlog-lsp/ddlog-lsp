pub async fn document_symbol(
    _tree: &tree_sitter::Tree,
    _params: lsp::DocumentSymbolParams,
    _content: &ropey::Rope,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    Ok(None)
}
