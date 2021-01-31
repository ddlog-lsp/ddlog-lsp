//! Parsers for the DDlog language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Core functionality for the DDlog language server.
pub mod core;

// The external C-based parsers generated by ddlog-lsp/tree-sitter-ddlog.
#[cfg(not(target_arch = "wasm32"))]
extern {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_ddlog_dat() -> tree_sitter_sys::Language;

    #[allow(dead_code)]
    #[doc(hidden)]
    fn tree_sitter_ddlog_dl() -> tree_sitter_sys::Language;
}
