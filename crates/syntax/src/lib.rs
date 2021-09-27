//! Parsers for the DDlog language server.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Functionality related to syntax errors.
pub mod error;

/// Functionality related to [`tree-sitter::Language`].
pub mod language;

/// Functionality related to [`tree-sitter::Node`].
pub mod node;

/// Functionality related to [`tree_sitter::Range`].
pub mod range;
