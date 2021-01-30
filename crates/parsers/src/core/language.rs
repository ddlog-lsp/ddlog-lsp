//! Core functionality related to document languages.

use crate::core::error::Error::{InvalidLanguageId, OsStrToStrFailed, PathExtensionFailed};
use std::{convert::TryFrom, path::Path};

/// Languages supported by the server.
#[derive(Clone, Copy, Debug)]
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
pub mod dat {
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unsafe_code)]
    /// Tree-sitter language for the `.dat` grammar.
    pub fn language() -> tree_sitter::Language {
        let inner = unsafe { crate::tree_sitter_ddlog_dat() };
        inner.into()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn language() -> tree_sitter::Language {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        let bytes: &[u8] = include_bytes!("../../../../vendor/tree-sitter-ddlog/ddlog/dat/tree-sitter-ddlog_dat.wasm");
        let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
        let future = JsFuture::from(promise);
        let result = futures::future::block_on(future).unwrap();
        let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
        inner.into()
    }
}

/// Functions for working with the `.dl` grammar.
pub mod dl {
    /// Tree-sitter language for the `.dl` grammar.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        let inner = unsafe { crate::tree_sitter_ddlog_dl() };
        inner.into()
    }

    #[cfg(target_arch = "wasm32")]
    #[allow(unsafe_code)]
    pub fn language() -> tree_sitter::Language {
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        let bytes: &[u8] = include_bytes!("../../../../vendor/tree-sitter-ddlog/ddlog/dl/tree-sitter-ddlog_dl.wasm");
        let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
        let future = JsFuture::from(promise);
        let result = futures::future::block_on(future).unwrap();
        let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
        inner.into()
    }
}
