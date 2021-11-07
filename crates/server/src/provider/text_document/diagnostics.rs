use crate::core::Language;

mod dat;
mod dl;

pub fn diagnostics(
    tree: &tree_sitter::Tree,
    uri: &lsp::Url,
    language: Language,
    content: &ropey::Rope,
) -> Vec<lsp::Diagnostic> {
    match language {
        crate::core::Language::DDlogDat => dat::diagnostics(tree, uri, content),
        crate::core::Language::DDlogDl => dl::diagnostics(tree, uri, content),
    }
}
