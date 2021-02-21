//! Core functionality related to document languages.

use crate::core::error::Error::{InvalidLanguageId, OsStrToStrFailed, PathExtensionFailed};
use std::{convert::TryFrom, path::Path};

/// Languages supported by the server.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    /// The `.dat` language.
    DDlogDat,
    /// The `.dl` language.
    DDlogDl,
}

impl Language {
    /// Compute the language id string for the given language.
    pub fn id(&self) -> &str {
        match self {
            Language::DDlogDat => "ddlog.dat",
            Language::DDlogDl => "ddlog.dl",
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;

    fn try_from(language_id: &str) -> anyhow::Result<Self> {
        match language_id {
            "ddlog.dat" => Ok(Language::DDlogDat),
            "ddlog.dl" => Ok(Language::DDlogDl),
            _ => Err(InvalidLanguageId(language_id.into()).into()),
        }
    }
}

impl TryFrom<&Path> for Language {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> anyhow::Result<Self> {
        let file_ext = path.extension().ok_or_else(|| PathExtensionFailed(path.into()))?;
        let file_ext = file_ext.to_str().ok_or(OsStrToStrFailed)?;
        let language_id = format!("ddlog.{}", file_ext);
        Language::try_from(language_id.as_str())
    }
}

/// Functions for working with the `.dat` grammar.
pub mod dat;

/// Functions for working with the `.dl` grammar.
pub mod dl;
