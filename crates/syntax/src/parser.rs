//! Core functionality related to document parsers.

use crate::language::{self, Language};
use std::convert::TryFrom;

/// Create a `.dat` parser from the tree-sitter grammar.
pub fn dat() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::dat::language();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

/// Create a `.dl` parser from the tree-sitter grammar.
pub fn dl() -> anyhow::Result<tree_sitter::Parser> {
    let language = language::dl::language();
    let mut parser = tree_sitter::Parser::new()?;
    parser.set_language(&language)?;
    Ok(parser)
}

impl TryFrom<Language> for tree_sitter::Parser {
    type Error = anyhow::Error;

    fn try_from(language: Language) -> anyhow::Result<tree_sitter::Parser> {
        match language {
            Language::DDlogDat => dat(),
            Language::DDlogDl => dl(),
        }
    }
}
