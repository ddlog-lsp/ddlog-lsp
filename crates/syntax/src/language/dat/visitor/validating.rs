use crate::{
    language::{dat::visitor::Visitor, HasWalker},
    node::NodeWalker,
};
use ddlog_lsp_languages::language::Language;

#[allow(missing_docs)]
pub struct ValidatingVisitor<'tree> {
    walker: NodeWalker<'tree>,
}

impl<'tree> ValidatingVisitor<'tree> {
    #[allow(missing_docs)]
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let walker = NodeWalker::new(language, node);
        Self { walker }
    }
}

impl<'tree> HasWalker<'tree> for ValidatingVisitor<'tree> {
    fn walker(&mut self) -> &mut NodeWalker<'tree> {
        &mut self.walker
    }
}

impl<'tree> Visitor<'tree> for ValidatingVisitor<'tree> {
}
