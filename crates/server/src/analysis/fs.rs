use futures::{
    future,
    stream::{self, StreamExt},
    Stream,
};

pub fn workspace_documents_paths(workspace_folder: &lsp::WorkspaceFolder) -> impl Stream<Item = std::path::PathBuf> {
    let init_path = workspace_folder
        .uri
        .to_file_path()
        .expect("valid workspace paths should always resolve to file paths");
    let init = vec![init_path];
    let f = |mut work: Vec<std::path::PathBuf>| async move {
        while let Some(path) = work.pop() {
            if path.is_dir() {
                if let Ok(read_dir) = tokio::fs::read_dir(path).await {
                    let read_dir_stream = tokio_stream::wrappers::ReadDirStream::new(read_dir);
                    let dir_entries = read_dir_stream
                        .filter_map(|read_dir| future::ready(Result::ok(read_dir)))
                        .map(|dir_entry| dir_entry.path())
                        .collect::<Vec<_>>()
                        .await;
                    work.extend(dir_entries);
                }
                continue;
            }
            if path.is_file() {
                let path_name = path.to_string_lossy();
                // Conditionally filter ".fail.dl" files in debug mode (used for corpus tests)
                #[cfg(debug_assertions)]
                if path_name.ends_with(".fail.dl") {
                    continue;
                }
                if path_name.ends_with(".dat") || path_name.ends_with(".dl") {
                    return Some((path, work));
                }
                continue;
            }
        }
        None
    };
    Box::pin(stream::unfold(init, f))
}
