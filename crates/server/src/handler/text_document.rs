use crate::core::EagerFutureExt;
use futures::future;
use lsp_text::RopeExt;
use std::sync::Arc;

pub async fn definition(
    session: Arc<crate::core::Session>,
    params: lsp::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp::GotoDefinitionResponse>> {
    crate::provider::text_document::definition(session, params).await
}

pub async fn did_change(
    session: Arc<crate::core::Session>,
    params: lsp::DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let uri = &params.text_document.uri;
    let mut text = session.get_mut_text(uri).await?;
    let mut content = text.get_content().await?;

    let edits = params
        .content_changes
        .iter()
        .map(|change| content.build_edit(change))
        .collect::<Result<Vec<_>, _>>()?;

    for edit in &edits {
        content.apply_edit(edit);
    }

    if let Some(tree) = crate::core::Document::change(session.clone(), uri, &content, &edits).await? {
        let diagnostics = crate::provider::text_document::diagnostics(&tree, uri, text.language, &content);
        let version = Default::default();
        session
            .client()?
            .publish_diagnostics(uri.clone(), diagnostics, version)
            .await;
    }

    text.set_content(future::ready(content).eager());

    Ok(())
}

pub async fn did_close(
    session: Arc<crate::core::Session>,
    params: lsp::DidCloseTextDocumentParams,
) -> anyhow::Result<()> {
    if session.document_workspaces.get(&params.text_document.uri).is_some() {
        let uri = params.text_document.uri.clone();
        let result = session.document_states.insert(uri, crate::core::DocumentState::Closed);
        debug_assert!(matches!(result, Some(crate::core::DocumentState::Opened)));
        return Ok(());
    }

    let uri = params.text_document.uri;
    session.remove_document(&uri)?;
    let diagnostics = Default::default();
    let version = Default::default();
    session.client()?.publish_diagnostics(uri, diagnostics, version).await;

    Ok(())
}

pub async fn did_open(
    session: Arc<crate::core::Session>,
    params: lsp::DidOpenTextDocumentParams,
) -> anyhow::Result<()> {
    if session.document_trees.contains_key(&params.text_document.uri) {
        let uri = params.text_document.uri.clone();
        let result = session.document_states.insert(uri, crate::core::DocumentState::Opened);
        debug_assert!(matches!(result, Some(crate::core::DocumentState::Closed)));
        return Ok(());
    }

    let workspace_folder = None;
    let uri = params.text_document.uri.clone();

    let document = crate::core::Document::open_from_lsp(params)?;
    let text = document.text();

    let content = text.get_content().await?;
    let tree = document
        .tree
        .clone()
        .await
        .ok_or_else(|| anyhow::anyhow!("could not open tree for uri: {:#?}", uri))?;

    session.insert_document(workspace_folder, document).await?;
    let diagnostics = {
        let tree = tree.lock().await;
        crate::provider::text_document::diagnostics(&tree, &uri, text.language, &content)
    };
    let version = Default::default();
    session.client()?.publish_diagnostics(uri, diagnostics, version).await;

    Ok(())
}

pub async fn document_symbol(
    session: Arc<crate::core::Session>,
    params: lsp::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
    crate::provider::text_document::document_symbol(session, params).await
}
