use crate::core;

mod dat;
mod dl;

pub fn diagnostics(tree: &tree_sitter::Tree, text: &core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        core::Language::DDlogDat => dat::diagnostics(tree, &text.content),
        core::Language::DDlogDl => dl::diagnostics(tree, &text.content),
    }
}
