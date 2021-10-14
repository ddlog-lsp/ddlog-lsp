use crate::{core, server};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use std::convert::TryFrom;

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
    texts: DashMap<lsp::Url, core::Text>,
    pub parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    pub trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
    pub workspace_folders: RwLock<Option<Vec<lsp::WorkspaceFolder>>>,
}

impl Session {
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let texts = Default::default();
        let parsers = Default::default();
        let trees = Default::default();
        let workspace_folders = RwLock::new(Default::default());
        Ok(Session {
            server_capabilities,
            client_capabilities,
            client,
            texts,
            parsers,
            trees,
            workspace_folders,
        })
    }

    pub fn client(&self) -> anyhow::Result<&lspower::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| core::Error::ClientNotInitialized.into())
    }

    pub fn insert_document(&self, uri: lsp::Url, document: core::Document) -> anyhow::Result<()> {
        let result = self.texts.insert(uri.clone(), document.text());
        debug_assert!(result.is_none());
        let result = self.parsers.insert(uri.clone(), Mutex::new(document.parser));
        debug_assert!(result.is_none());
        let result = self.trees.insert(uri, Mutex::new(document.tree));
        debug_assert!(result.is_none());
        Ok(())
    }

    pub async fn read_documents(&self) -> anyhow::Result<Vec<(lsp::Url, core::Document)>> {
        fn walk_folder(
            uri: &lsp::Url,
            folder_path: &std::path::Path,
            results: &mut Vec<(lsp::Url, core::Document)>,
        ) -> anyhow::Result<()> {
            if !folder_path.is_dir() {
                return Ok(());
            }

            for entry in folder_path.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_folder(uri, path.as_path(), results)?;
                } else {
                    if path.to_string_lossy().ends_with(".fail.dl") {
                        continue;
                    }
                    if let Ok(language) = core::Language::try_from(&*path) {
                        let uri = lsp::Url::from_file_path(&path).unwrap();
                        let params = lsp::DidOpenTextDocumentParams {
                            text_document: {
                                let language_id = language.id().into();
                                let version = Default::default();
                                let text = std::fs::read_to_string(path)?;
                                lsp::TextDocumentItem {
                                    uri: uri.clone(),
                                    language_id,
                                    version,
                                    text,
                                }
                            },
                        };
                        if let Some(document) = core::Document::open(params)? {
                            results.push((uri, document));
                        } else {
                            // FIXME: should handle parse errors or just ignore them?
                        }
                    }
                }
            }

            Ok(())
        }

        let mut results = vec![];
        if let Some(workspace_folders) = &*self.workspace_folders.read().await {
            for folder in workspace_folders {
                if let Ok(folder_path) = folder.uri.to_file_path() {
                    walk_folder(&folder.uri, folder_path.as_path(), &mut results)?;
                }
            }
        }
        Ok(results)
    }

    pub fn remove_document(&self, uri: &lsp::Url) -> anyhow::Result<()> {
        let result = self.texts.remove(uri);
        debug_assert!(result.is_some());
        let result = self.parsers.remove(uri);
        debug_assert!(result.is_some());
        let result = self.trees.remove(uri);
        debug_assert!(result.is_some());
        Ok(())
    }

    pub async fn semantic_tokens_legend(&self) -> Option<lsp::SemanticTokensLegend> {
        let capabilities = self.server_capabilities.read().await;
        if let Some(capabilities) = &capabilities.semantic_tokens_provider {
            match capabilities {
                lsp::SemanticTokensServerCapabilities::SemanticTokensOptions(options) => Some(options.legend.clone()),
                lsp::SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(options) => {
                    Some(options.semantic_tokens_options.legend.clone())
                }
            }
        } else {
            None
        }
    }

    pub async fn get_text(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, core::Text>> {
        self.texts.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, core::Text>> {
        self.texts.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }
}
