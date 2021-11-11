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
    let query_patterns = params.query.split(' ').collect::<Vec<_>>();
    let mut results = vec![];
    for item in session.document_symbols.iter() {
        for info in item.value().clone().await.unwrap_or_default() {
            if query_patterns
                .iter()
                .all(|pattern| twoway::find_str(info.name.as_str(), pattern).is_some())
            {
                results.push(info.clone());
            }
        }
    }
    Ok(Some(results))
}
