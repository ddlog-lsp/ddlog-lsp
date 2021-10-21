use crate::core;

mod dat;
mod dl;

pub fn diagnostics(tree: &tree_sitter::Tree, uri: &lsp::Url, text: &core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        core::Language::DDlogDat => dat::diagnostics(tree, uri, &text.content),
        core::Language::DDlogDl => dl::diagnostics(tree, uri, &text.content),
    }
}
