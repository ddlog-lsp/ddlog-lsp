//! Provides `textDocument/documentSymbol` functionality.

use crate::core::document::Document;
use tower_lsp::lsp_types::*;

// Compute diagnostics for a change event given a `document` and `tree`.
fn for_change(_document: &Document, tree: tree_sitter::Tree) -> anyhow::Result<Vec<Diagnostic>> {
    let mut diagnostics = vec![];
    let mut work = vec![tree.root_node()];
    let mut cursor = tree.root_node().walk();

    while let Some(node) = work.pop() {
        if node.is_error() {
            let message = String::from("ERROR node");
            let range = crate::util::node::range(&node);
            let severity = Some(DiagnosticSeverity::Error);
            diagnostics.push(Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            continue;
        }
        if node.is_missing() {
            let message = String::from("MISSING node");
            let range = crate::util::node::range(&node);
            let severity = Some(DiagnosticSeverity::Error);
            diagnostics.push(Diagnostic {
                message,
                range,
                severity,
                ..Default::default()
            });
            continue;
        }
        if node.has_error() {
            cursor.reset(node);
            work.extend(node.named_children(&mut cursor));
        }
    }

    Ok(diagnostics)
}

/// Functions related to processing parse tree events for a document.
pub(crate) mod tree {
    use crate::core::session::Session;
    use std::sync::Arc;
    use tower_lsp::lsp_types::*;

    /// Handle a parse tree "change" event.
    pub(crate) async fn change(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        let document = session.get_document(&uri).await?;
        let tree = document.tree.lock().await.clone();
        let diagnostics = super::for_change(&document, tree)?;
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// Handle a parse tree "close" event.
    pub(crate) async fn close(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        // clear diagnostics on tree close
        // FIXME: handle this properly
        let diagnostics = Default::default();
        let version = Default::default();
        session.client()?.publish_diagnostics(uri, diagnostics, version).await;
        Ok(())
    }

    /// Handle a parse tree "open" event.
    pub(crate) async fn open(session: Arc<Session>, uri: Url) -> anyhow::Result<()> {
        self::change(session, uri).await
    }
}
