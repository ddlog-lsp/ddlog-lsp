//! The DDlog language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Command-line interface for the DDlog language server.
pub mod cli;

// Core functionality for the DDlog language server.
pub mod core;

// Functionality related to implementation of the Language Server Protocol (LSP).
pub mod lsp;

// Definitions related to the ddlog-lsp crate.
mod package;

// Providers of the DDlog language server for LSP features.
pub mod provider;

// Services (components) of the DDlog language server.
pub mod service;

// Various utility functionality, e.g., for handling lsp or tree-sitter data.
mod util;
