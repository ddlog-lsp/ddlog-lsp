pub async fn document_symbol(
    _content: &ropey::Rope,
    _tree: &tree_sitter::Tree,
    _params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    Ok(None)
}
