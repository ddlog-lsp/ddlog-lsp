use crate::{core, server};
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
    DashSet,
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
    workspace_folders: DashMap<core::WorkspaceFolder, DashSet<lsp::Url>>,
}

impl Session {
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let texts = DashMap::default();
        let parsers = DashMap::default();
        let trees = DashMap::default();
        let workspace_folders = DashMap::default();
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
                },
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

    pub fn insert_workspace_folders(&self, workspace_folders: Vec<lsp::WorkspaceFolder>) -> anyhow::Result<()> {
        for folder in workspace_folders {
            if let Ok(folder_path) = folder.uri.to_file_path() {
                let mut folder_entries = vec![];
                walk_folder(folder_path, &mut folder_entries)?;

                let mut workspace_document_uris: DashSet<lsp::Url> = DashSet::new();
                for (uri, document) in folder_entries {
                    workspace_document_uris.insert(uri.clone());
                    self.insert_document(uri, document);
                }

                self.workspace_folders
                    .insert(core::WorkspaceFolder(folder), workspace_document_uris);
            }
        }
        Ok(())
    }

    pub fn remove_workspace_folders(&self, workspace_folders: Vec<lsp::WorkspaceFolder>) {
        for folder in workspace_folders {
            self.workspace_folders.remove(&core::WorkspaceFolder(folder));
        }
    }
}

fn walk_folder(
    current_subfolder: std::path::PathBuf,
    results: &mut Vec<(lsp::Url, core::Document)>,
) -> anyhow::Result<()> {
    if !current_subfolder.is_dir() {
        return Ok(());
    }

    for entry in current_subfolder.read_dir()? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            walk_folder(entry_path, results)?;
        } else {
            if entry_path.to_string_lossy().ends_with(".fail.dl") {
                continue;
            }
            if let Ok(language) = core::Language::try_from(&*entry_path) {
                let uri = lsp::Url::from_file_path(&entry_path).unwrap();
                let params = lsp::DidOpenTextDocumentParams {
                    text_document: {
                        let language_id = language.id().into();
                        let version = Default::default();
                        let text = std::fs::read_to_string(entry_path)?;
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
