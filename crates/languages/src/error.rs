use std::path::PathBuf;
use thiserror::Error;

/// Runtime errors for the DDlog parsers.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    /// Error that occurs when parsing an invalid language-id string.
    #[error("InvalidLanguageId: {0}")]
    InvalidLanguageId(String),
    /// Error that occurs when `OsStr::to_str()` returns `None`.
    #[error("OsStrToStrFailed")]
    OsStrToStrFailed,
    /// Error that occurs when `Path::extension()` returns `None`.
    #[error("PathExtensionFailed: {0}")]
    PathExtensionFailed(PathBuf),
}
