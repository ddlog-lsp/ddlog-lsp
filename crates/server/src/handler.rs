pub mod text_document {
    use crate::{core, provider};
    use std::sync::Arc;

    pub async fn did_change(
        session: Arc<core::Session>,
        params: lsp::DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        {
            let mut text = session.get_mut_text(uri).await?;
            for change in &params.content_changes {
                if change.range.is_some() {
                    let edit = text.build_edit(change)?;
                    text.apply_edit(&edit);
                } else {
                    text.content = ropey::Rope::from(change.text.as_str());
                    break;
                }
            }
        }
        let text = session.get_text(uri).await?;
        if let Some(tree) = core::Document::change(session.clone(), uri).await? {
            let diagnostics = provider::diagnostics(&tree, &text.content);
            let version = Default::default();
            session
                .client()?
                .publish_diagnostics(uri.clone(), diagnostics, version)
                .await;
        }
        Ok(())
    }

    pub async fn did_close(session: Arc<core::Session>, params: lsp::DidCloseTextDocumentParams) -> anyhow::Result<()> {
        let uri = params.text_document.uri;
        session.remove_document(&uri)?;
        let diagnostics = Default::default();
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    pub async fn did_open(session: Arc<core::Session>, params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let uri = params.text_document.uri.clone();
        if let Some(document) = core::Document::open(params)? {
            let tree = document.tree.clone();
            let text = document.text();
            session.insert_document(uri.clone(), document)?;
            let diagnostics = provider::diagnostics(&tree, &text.content);
            let version = Default::default();
            session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        } else {
            log::warn!("'textDocument/didOpen' failed :: uri: {:#?}", uri);
        }
        Ok(())
    }

    pub async fn document_symbol(
        _session: Arc<core::Session>,
        _params: lsp::DocumentSymbolParams,
    ) -> anyhow::Result<Option<lsp::DocumentSymbolResponse>> {
        todo!()
    }
}
