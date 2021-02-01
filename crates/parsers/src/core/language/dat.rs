//! Functions for working with the `.dat` grammar.

/// Tree-sitter language for the `.dat` grammar.
#[cfg(not(target_arch = "wasm32"))]
#[allow(unsafe_code)]
pub fn language() -> tree_sitter::Language {
    let inner = unsafe { crate::tree_sitter_ddlog_dat() };
    inner.into()
}

/// Tree-sitter language for the `.dat` grammar.
#[cfg(target_arch = "wasm32")]
pub fn language() -> tree_sitter::Language {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let bytes: &[u8] = include_bytes!("../../../../../vendor/tree-sitter-ddlog/ddlog/dat/tree-sitter-ddlog_dat.wasm");
    let promise = web_tree_sitter_sys::Language::load_bytes(&bytes.into());
    let future = JsFuture::from(promise);
    let result = futures::future::block_on(future).unwrap();
    let inner = result.unchecked_into::<web_tree_sitter_sys::Language>();
    inner.into()
}
