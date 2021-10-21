mod dat;
mod dl;

pub fn diagnostics(tree: &tree_sitter::Tree, uri: &lsp::Url, text: &crate::core::Text) -> Vec<lsp::Diagnostic> {
    match text.language {
        crate::core::Language::DDlogDat => dat::diagnostics(tree, uri, &text.content),
        crate::core::Language::DDlogDl => dl::diagnostics(tree, uri, &text.content),
    }
}
