pub mod dat;
pub mod dl;

pub async fn document_symbol(
    text: crate::core::Text,
    tree: &tree_sitter::Tree,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Vec<lsp::SymbolInformation>> {
    // let text = session.get_text(&params.text_document.uri).await?;
    let content = text.get_content().await?;
    let response = match text.language {
        crate::core::Language::DDlogDat => self::dat::document_symbol(&content, tree, params).await?,
        crate::core::Language::DDlogDl => self::dl::document_symbol(&content, tree, params).await?,
    };
    Ok(response)
}

pub async fn document_symbol_from_uri(
    text: crate::core::Text,
    tree: &tree_sitter::Tree,
    uri: lsp::Url,
) -> anyhow::Result<Vec<lsp::SymbolInformation>> {
    let params = lsp::DocumentSymbolParams {
        text_document: lsp::TextDocumentIdentifier { uri },
        partial_result_params: Default::default(),
        work_done_progress_params: Default::default(),
    };
    document_symbol(text, tree, params).await
}
