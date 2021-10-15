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
    workspace_documents: DashMap<core::WorkspaceFolder, DashSet<lsp::Url>>,
    pub document_workspaces: DashMap<lsp::Url, core::WorkspaceFolder>,
    pub document_states: DashMap<lsp::Url, core::DocumentState>,
    document_texts: DashMap<lsp::Url, core::Text>,
    pub document_parsers: DashMap<lsp::Url, Mutex<tree_sitter::Parser>>,
    pub document_trees: DashMap<lsp::Url, Mutex<tree_sitter::Tree>>,
}

impl Session {
    pub fn new(client: Option<lspower::Client>) -> anyhow::Result<Self> {
        let server_capabilities = RwLock::new(server::capabilities());
        let client_capabilities = RwLock::new(Default::default());
        let workspace_documents = DashMap::default();
        let document_workspaces = DashMap::default();
        let document_states = DashMap::default();
        let document_texts = DashMap::default();
        let document_parsers = DashMap::default();
        let document_trees = DashMap::default();
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
        })
    }

    pub fn client(&self) -> anyhow::Result<&lspower::Client> {
        self.client
            .as_ref()
            .ok_or_else(|| core::Error::ClientNotInitialized.into())
    }

    pub fn insert_document(
        &self,
        workspace_folder: Option<core::WorkspaceFolder>,
        document: core::Document,
    ) -> anyhow::Result<()> {
        // create document_workspaces entry
        if let Some(workspace_folder) = workspace_folder {
            let result = self.document_workspaces.insert(document.uri.clone(), workspace_folder);
            debug_assert!(result.is_none());
        }

        // create document_states entry
        let result = self
            .document_states
            .insert(document.uri.clone(), core::DocumentState::Closed);
        debug_assert!(result.is_none());

        // create document_texts entry
        let result = self.document_texts.insert(document.uri.clone(), document.text());
        debug_assert!(result.is_none());

        // create document_parsers entry
        let result = self
            .document_parsers
            .insert(document.uri.clone(), Mutex::new(document.parser));
        debug_assert!(result.is_none());

        // create document_trees entry
        let result = self.document_trees.insert(document.uri, Mutex::new(document.tree));
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
        self.document_texts.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_text(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, core::Text>> {
        self.document_texts.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Document;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_parser(
        &self,
        uri: &lsp::Url,
    ) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Parser>>> {
        self.document_parsers.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Parser;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_tree(&self, uri: &lsp::Url) -> anyhow::Result<Ref<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.document_trees.get(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub async fn get_mut_tree(&self, uri: &lsp::Url) -> anyhow::Result<RefMut<'_, lsp::Url, Mutex<tree_sitter::Tree>>> {
        self.document_trees.get_mut(uri).ok_or_else(|| {
            let kind = SessionResourceKind::Tree;
            let uri = uri.clone();
            core::Error::SessionResourceNotFound { kind, uri }.into()
        })
    }

    pub fn insert_workspace_folders(&self, workspaces: Vec<lsp::WorkspaceFolder>) -> anyhow::Result<()> {
        for workspace_folder in workspaces {
            if let Ok(workspace_folder_path) = workspace_folder.uri.to_file_path() {
                let mut workspace_folder_entries = vec![];
                walk_folder(workspace_folder_path, &mut workspace_folder_entries)?;

                let mut workspace_document_uris: DashSet<lsp::Url> = DashSet::new();
                for document in workspace_folder_entries {
                    workspace_document_uris.insert(document.uri.clone());
                    let workspace_folder = Some(core::WorkspaceFolder(workspace_folder.clone()));
                    self.insert_document(workspace_folder, document);
                }

                self.workspace_documents
                    .insert(core::WorkspaceFolder(workspace_folder), workspace_document_uris);
            }
        }
        Ok(())
    }

    pub fn remove_workspace_folders(&self, workspace_folders: Vec<lsp::WorkspaceFolder>) {
        for folder in workspace_folders {
            self.workspace_documents.remove(&core::WorkspaceFolder(folder));
        }
    }
}

fn walk_folder(current_subfolder: std::path::PathBuf, results: &mut Vec<core::Document>) -> anyhow::Result<()> {
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
                            uri,
                            language_id,
                            version,
                            text,
                        }
                    },
                };
                if let Some(document) = core::Document::open(params)? {
                    results.push(document);
                } else {
                    // FIXME: should handle parse errors or just ignore them?
                }
            }
        }
    }

    Ok(())
}
