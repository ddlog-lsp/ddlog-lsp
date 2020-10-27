//! Core functionality for the DDlog language server.

// Core functionality related to documents.
pub mod document;

// Core functionality related to runtime errors.
pub(crate) mod error;

// Core functionality related to parsers.
pub(crate) use ddlog_lsp_parsers::core::*;

// Core functionality related to the LSP server session.
pub mod session;
