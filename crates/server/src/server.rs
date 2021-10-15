use crate::{core, handler};
use lspower::jsonrpc;
use sorted_vec::SortedVec;
use std::sync::Arc;

pub struct Server {
    pub client: lspower::Client,
    pub session: Arc<core::Session>,
}

impl Server {
    pub fn new(client: lspower::Client) -> anyhow::Result<Self> {
        let session = Arc::new(core::Session::new(Some(client.clone()))?);
        Ok(Server { client, session })
    }
}

pub fn capabilities() -> lsp::ServerCapabilities {
    let document_symbol_provider = Some(lsp::OneOf::Left(true));

    let text_document_sync = {
        let options = lsp::TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(lsp::TextDocumentSyncKind::Incremental),
            ..Default::default()
        };
        Some(lsp::TextDocumentSyncCapability::Options(options))
    };

    let workspace = {
        let options = lsp::WorkspaceServerCapabilities {
            workspace_folders: Some(lsp::WorkspaceFoldersServerCapabilities {
                change_notifications: Some(lsp::OneOf::Left(true)),
                supported: Some(true),
            }),
            ..Default::default()
        };
        Some(options)
    };

    lsp::ServerCapabilities {
        text_document_sync,
        document_symbol_provider,
        workspace,
        ..Default::default()
    }
}

#[lspower::async_trait]
impl lspower::LanguageServer for Server {
    async fn initialize(&self, params: lsp::InitializeParams) -> jsonrpc::Result<lsp::InitializeResult> {
        *self.session.client_capabilities.write().await = Some(params.capabilities);
        *self.session.workspace_folders.write().await = {
            let mut sorted = SortedVec::new();
            if let Some(workspace_folders) = params.workspace_folders {
                for folder in workspace_folders {
                    sorted.insert(core::WorkspaceFolder(folder));
                }
                Some(sorted)
            } else {
                None
            }
        };
        let capabilities = capabilities();
        Ok(lsp::InitializeResult {
            capabilities,
            ..lsp::InitializeResult::default()
        })
    }

    async fn initialized(&self, _: lsp::InitializedParams) {
        let typ = lsp::MessageType::Info;
        let message = "DDlog language server initialized!";
        self.client.log_message(typ, message).await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: lsp::DidOpenTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_open(session, params).await.unwrap()
    }

    async fn did_change(&self, params: lsp::DidChangeTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_change(session, params).await.unwrap()
    }

    async fn did_change_workspace_folders(&self, params: lsp::DidChangeWorkspaceFoldersParams) {
        let session = self.session.clone();
        handler::workspace::did_change_workspace_folders(session, params).await
    }

    async fn did_close(&self, params: lsp::DidCloseTextDocumentParams) {
        let session = self.session.clone();
        handler::text_document::did_close(session, params).await.unwrap()
    }

    async fn document_symbol(
        &self,
        params: lsp::DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<lsp::DocumentSymbolResponse>> {
        let session = self.session.clone();
        let result = handler::text_document::document_symbol(session, params).await;
        Ok(result.map_err(core::IntoJsonRpcError)?)
    }
}
