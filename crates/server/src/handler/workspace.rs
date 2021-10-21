use crate::core;
use std::sync::Arc;

pub async fn did_change_workspace_folders(
    session: Arc<core::Session>,
    params: lsp::DidChangeWorkspaceFoldersParams,
) -> anyhow::Result<()> {
    session.remove_workspace_folders(params.event.removed);
    session.insert_workspace_folders(params.event.added)?;
    Ok(())
}
