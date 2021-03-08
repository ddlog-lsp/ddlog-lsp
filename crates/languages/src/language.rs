//! Functions for creating [`tree-sitter::Language`].

use crate::error::Error;
use std::{convert::TryFrom, path::Path};

/// Tree-sitter language for the `.dat` grammar.
#[cfg(not(target_arch = "wasm32"))]
pub fn dat() -> tree_sitter::Language {
    #[allow(unsafe_code)]
    let inner = unsafe { crate::tree_sitter_ddlog_dat() };
    inner.into()
}

/// Tree-sitter language for the `.dat` grammar.
#[cfg(target_arch = "wasm32")]
pub fn dat() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../vendor/tree-sitter-ddlog/ddlog/dat/tree-sitter-ddlog_dat.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::future::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}

/// Tree-sitter language for the `.dl` grammar.
#[cfg(not(target_arch = "wasm32"))]
pub fn dl() -> tree_sitter::Language {
    #[allow(unsafe_code)]
    let inner = unsafe { crate::tree_sitter_ddlog_dl() };
    inner.into()
}

/// Tree-sitter language for the `.dl` grammar.
#[cfg(target_arch = "wasm32")]
pub fn dl() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../vendor/tree-sitter-ddlog/ddlog/dl/tree-sitter-ddlog_dl.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::future::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}

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
            _ => Err(Error::InvalidLanguageId(language_id.into()).into()),
        }
    }
}

impl TryFrom<&Path> for Language {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> anyhow::Result<Self> {
        let file_ext = path
            .extension()
            .ok_or_else(|| Error::PathExtensionFailed(path.into()))?;
        let file_ext = file_ext.to_str().ok_or(Error::OsStrToStrFailed)?;
        let language_id = format!("ddlog.{}", file_ext);
        Language::try_from(language_id.as_str())
    }
}
