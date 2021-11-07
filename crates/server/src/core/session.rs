use crate::core::future::EagerFuture;
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
    DashSet,
};
use futures::{future, stream::StreamExt};
use std::sync::Arc;

#[cfg(feature = "runtime-agnostic")]
use async_lock::{Mutex, RwLock};
#[cfg(feature = "tokio")]
use tokio::sync::{Mutex, RwLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionResourceKind {
    Document,
    Parser,
    Tree,
}

pub struct Session {
    pub server_capabilities: RwLock<lsp::ServerCapabilities>,
    pub client_capabilities: RwLock<Option<lsp::ClientCapabilities>>,
    client: Option<lspower::Client>,
    pub workspace_documents: DashMap<crate::core::WorkspaceFolder, DashSet<lsp::Url>>,
    pub document_workspaces: DashMap<lsp::Url, crate::core::WorkspaceFolder>,
    pub document_states: DashMap<lsp::Url, crate::core::DocumentState>,
    document_texts: DashMap<lsp::Url, crate::core::Text>,
    pub document_parsers: DashMap<lsp::Url, Arc<Mutex<tree_sitter::Parser>>>,
    pub document_trees: DashMap<lsp::Url, EagerFuture<Option<Arc<Mutex<tree_sitter::Tree>>>>>,
    pub document_symbols: DashMap<lsp::Url, Vec<lsp::SymbolInformation>>,
}

impl Session {
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(crate::server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let workspace_documents = DashMap::default();
        let document_workspaces = DashMap::default();
        let document_states = DashMap::default();
        let document_texts = DashMap::default();
        let document_parsers = DashMap::default();
        let document_trees = DashMap::default();
        let document_symbols = DashMap::default();
        Ok(Session {
            server_capabilities,
            client_capabilities,
            client,
            workspace_documents,
            document_workspaces,
            document_states,
            document_texts,
            document_parsers,
            document_trees,
            document_symbols,
        })
    }
}

impl Session {
    pub fn client(&self) -> anyhow::Result<&lspower::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| crate::core::Error::ClientNotInitialized.into())
    }
}

impl Session {
    pub async fn insert_document(
        &self,
        workspace_folder: Option<crate::core::WorkspaceFolder>,
        document: crate::core::Document,
    ) -> anyhow::Result<()> {
        let uri = &document.uri;

        // create document_workspaces entry
        if let Some(workspace_folder) = workspace_folder {
            let result = self.document_workspaces.insert(uri.clone(), workspace_folder);
            debug_assert!(result.is_none());
        }

        // create document_states entry
        let result = self
            .document_states
            .insert(document.uri.clone(), crate::core::DocumentState::Closed);
        debug_assert!(result.is_none());

        // create document_texts entry
        let result = self.document_texts.insert(uri.clone(), document.text());
        debug_assert!(result.is_none());

        // create document_parsers entry
        let result = self.document_parsers.insert(document.uri.clone(), document.parser);
        debug_assert!(result.is_none());

        // create document_trees entry
        let result = self.document_trees.insert(uri.clone(), document.tree);
        debug_assert!(result.is_none());

        // create document_symbols entry
        let result = {
            let key = uri.clone();
            let value = crate::analysis::document_symbol_from_uri(self, key.clone()).await?;
            self.document_symbols.insert(key, value)
        };
        debug_assert!(result.is_none());

        Ok(())
    }

    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        // delete document_workspaces entry
        self.document_workspaces.remove(uri);

        // delete document_states entry
        let result = self.document_states.remove(uri);
        debug_assert!(result.is_some());

        // delete document_texts entry
        let result = self.document_texts.remove(uri);
        debug_assert!(result.is_some());

        // delete document_parsers entry
        let result = self.document_parsers.remove(uri);
        debug_assert!(result.is_some());

        // delete document_trees entry
        let result = self.document_trees.remove(uri);
        debug_assert!(result.is_some());

        // delete document_symbols entry
        let result = self.document_symbols.remove(uri);
        debug_assert!(result.is_some());

        Ok(())
    }
}

impl Session {
    pub async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                },
            }
        } else {
            None
        }
    }
}

impl Session {
    pub async fn get_text(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, crate::core::Text>> {
        self.document_texts.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, crate::core::Text>> {
        self.document_texts.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}

impl Session {
    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Arc<Mutex<tree_sitter::Parser>>>> {
        self.document_parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}

impl Session {
    pub async fn get_tree(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<Ref<'_, lsp::Url, EagerFuture<Option<Arc<Mutex<tree_sitter::Tree>>>>>> {
        self.document_trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, EagerFuture<Option<Arc<Mutex<tree_sitter::Tree>>>>>> {
        self.document_trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            crate::core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}

impl Session {
    pub async fn insert_workspace_folders(&self, workspaces: Vec<lsp::WorkspaceFolder>) -> anyhow::Result<()> {
        for workspace_folder in workspaces {
            if let Some(workspace_document_uris) = self.collect_workspace_document_uris(&workspace_folder).await? {
                for item in workspace_document_uris.iter() {
                    let document = crate::core::Document::open_from_uri(item.key().clone())?;
                    let workspace_folder = workspace_folder.clone();
                    let workspace_folder = crate::core::workspace_folder::WorkspaceFolder(workspace_folder);
                    let workspace_folder = Some(workspace_folder);
                    self.insert_document(workspace_folder, document).await?;
                }
                self.workspace_documents
                    .insert(crate::core::WorkspaceFolder(workspace_folder), workspace_document_uris);
            }
        }
        Ok(())
    }

    // FIXME: needs to do additional clean up work to reverse insert_workspace_folders
    pub fn remove_workspace_folders(&self, workspace_folders: Vec<lsp::WorkspaceFolder>) {
        for folder in workspace_folders {
            self.workspace_documents.remove(&crate::core::WorkspaceFolder(folder));
        }
    }

    async fn collect_workspace_document_uris(
        &self,
        workspace_folder: &lsp::WorkspaceFolder,
    ) -> anyhow::Result<Option<DashSet<lsp::Url>>> {
        if let Ok(workspace_folder_path) = workspace_folder.uri.to_file_path() {
            let mut work_stack = vec![workspace_folder_path];
            let mut workspace_document_uris = DashSet::new();

            while let Some(entry) = work_stack.pop() {
                if entry.is_dir() {
                    let read_dir = tokio::fs::read_dir(entry).await?;
                    let read_dir_stream = tokio_stream::wrappers::ReadDirStream::new(read_dir);
                    let dir_entries = read_dir_stream
                        .filter_map(|read_dir| future::ready(Result::ok(read_dir)))
                        .map(|dir_entry| dir_entry.path())
                        .collect::<Vec<_>>()
                        .await;
                    work_stack.extend(dir_entries);
                    continue;
                }
                if entry.is_file() {
                    let entry_name = entry.to_string_lossy();

                    // Conditionally filter ".fail.dl" files in debug mode (used for corpus tests)
                    #[cfg(debug_assertions)]
                    if entry_name.ends_with(".fail.dl") {
                        continue;
                    }

                    if entry_name.ends_with(".dat") || entry_name.ends_with(".dl") {
                        let uri = lsp::Url::from_file_path(&entry).unwrap();
                        workspace_document_uris.insert(uri);
                    }
                    continue;
                }
            }

            Ok(Some(workspace_document_uris))
        } else {
            Ok(None)
        }
    }
}
