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
    let results = session
        .document_symbols
        .iter()
        .flat_map(|item| {
            item.value()
                .iter()
                .filter(|info| {
                    query_patterns
                        .iter()
                        .all(|pattern| twoway::find_str(info.name.as_str(), pattern).is_some())
                })
                .cloned()
                .collect::<Vec<lsp::SymbolInformation>>()
        })
        .collect::<Vec<_>>();
    Ok(Some(results))
}
