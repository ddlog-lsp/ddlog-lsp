use crate::core::language::dl::{TrivialVisitor, Visitor};

pub fn diagnostics(tree: &tree_sitter::Tree, uri: &lsp::Url, content: &ropey::Rope) -> Vec<lsp::Diagnostic> {
    let mut diagnostics = vec![];
    let mut visitor = {
        let language = crate::core::Language::DDlogDl;
        let node = tree.root_node();
        TrivialVisitor::new(language, node)
    };
    let result = visitor.visit();

    if let Err(error) = result {
        let diagnostic = error.to_lsp_diagnostic(uri, content);
        diagnostics.push(diagnostic);
    }

    diagnostics
}
