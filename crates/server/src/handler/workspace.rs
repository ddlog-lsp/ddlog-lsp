use std::sync::Arc;

pub async fn did_change_workspace_folders(
    session: Arc<crate::core::Session>,
    params: lsp::DidChangeWorkspaceFoldersParams,
) -> anyhow::Result<()> {
    session.remove_workspace_folders(params.event.removed);
    session.insert_workspace_folders(params.event.added).await?;
    Ok(())
}

pub async fn symbol(
    session: Arc<crate::core::Session>,
    params: lsp::WorkspaceSymbolParams,
) -> anyhow::Result<Option<Vec<lsp::SymbolInformation>>> {
    crate::provider::workspace::symbol(session, params).await
}
