//! The DDlog language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    ddlog_lsp_server::cli::cli();

    let (service, messages) = LspService::new(|client| ddlog_lsp_server::lsp::server::Server::new(client).unwrap());
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout).interleave(messages).serve(service).await;

    Ok(())
}
