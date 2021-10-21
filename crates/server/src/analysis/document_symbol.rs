pub mod dat;
pub mod dl;

pub async fn document_symbol(
    session: &crate::core::Session,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Vec<lsp::SymbolInformation>> {
    let text = session.get_text(&params.text_document.uri).await?;
    let session = session.clone();
    let response = match text.language {
        crate::core::Language::DDlogDat => self::dat::document_symbol(session, params, &text.content).await?,
        crate::core::Language::DDlogDl => self::dl::document_symbol(session, params, &text.content).await?,
    };
    Ok(response)
}

pub async fn document_symbol_from_uri(
    session: &crate::core::Session,
    uri: lsp::Url,
) -> anyhow::Result<Vec<lsp::SymbolInformation>> {
    let params = lsp::DocumentSymbolParams {
        text_document: lsp::TextDocumentIdentifier { uri },
        partial_result_params: Default::default(),
        work_done_progress_params: Default::default(),
    };
    document_symbol(session, params).await
}
