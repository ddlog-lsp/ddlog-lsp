//! Functionality related to [`tree-sitter::Node`].

mod walker;

pub use walker::*;

#[allow(missing_docs)]
pub trait NodeExt<'tree> {
    fn first_child(&self) -> Option<tree_sitter::Node<'tree>>;
    fn next(&self) -> Option<tree_sitter::Node<'tree>>;
    fn next_ancestor_sibling(&self) -> Option<tree_sitter::Node<'tree>>;
}

impl<'tree> NodeExt<'tree> for tree_sitter::Node<'tree> {
    fn first_child(&self) -> Option<tree_sitter::Node<'tree>> {
        self.child(0)
    }

    fn next(&self) -> Option<tree_sitter::Node<'tree>> {
        let mut next;

        next = self.first_child();

        if next.is_none() {
            next = self.next_sibling();
            if next.is_none() {
                next = self.next_ancestor_sibling();
            }
        }

        next
    }

    fn next_ancestor_sibling(&self) -> Option<tree_sitter::Node<'tree>> {
        let mut next = Some(*self);

        loop {
            next = next.map(|node| node.parent()).flatten();
            if let Some(parent) = next {
                if let Some(parent_sibling) = parent.next_sibling() {
                    next = Some(parent_sibling);
                    break;
                }
            } else {
                break;
            }
        }

        next
    }
}
