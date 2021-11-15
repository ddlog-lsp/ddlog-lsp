use std::sync::Arc;

// FIXME: split functionality for {.dl, .dat}

pub async fn definition(
    session: Arc<crate::core::Session>,
    params: lsp::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp::GotoDefinitionResponse>> {
    use crate::analysis::imports;
    let origin_module_uri = &params.text_document_position_params.text_document.uri;
    if let Some(tree) = session.get_tree(origin_module_uri).await?.clone().await {
        let tree = tree.lock().await;
        let text = session.get_text(origin_module_uri).await?;
        let content = text.get_content().await?;
        let base = origin_module_uri.clone();
        let _imports = imports::collect_imports(&content, &tree).map(imports::resolve_import(base));
        todo!()
    } else {
        todo!()
    }
}
