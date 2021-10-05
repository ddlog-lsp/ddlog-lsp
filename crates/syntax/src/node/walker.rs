use crate::{
    error::SyntaxError,
    language::{dat, dl, NodeMove},
    node::NodeExt,
};
use ddlog_lsp_languages::language::Language;

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum StepValue<'tree> {
    Done,
    None,
    Some(tree_sitter::Node<'tree>),
}

#[allow(missing_docs)]
pub struct NodeWalker<'tree> {
    language: Language,
    node: tree_sitter::Node<'tree>,
    pub done: bool,
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GotoNext {
    StepInto,
    StepOver,
}

impl<'tree> NodeWalker<'tree> {
    /// Create a new [NodeWalker].
    pub fn new(language: Language, node: tree_sitter::Node<'tree>) -> Self {
        let done = Default::default();
        Self { language, node, done }
    }

    /// Move the cursor to the first child node.
    pub fn goto_first_child(&mut self) -> bool {
        let mut moved = false;

        if let Some(node) = self.node().first_child() {
            self.node = node;
            moved = true;
        }

        moved
    }

    /// Move the cursor to the next sibling node.
    pub fn goto_next_sibling(&mut self) -> bool {
        let mut moved = false;

        if let Some(node) = self.node().next_sibling() {
            self.node = node;
            moved = true;
        }

        moved
    }

    /// Move cursor to the next accessible node.
    pub fn goto_next(&mut self, goto_next: GotoNext, skip_extras: bool) -> bool {
        let mut moved = false;

        // First try to descend to the first child node.
        if goto_next == GotoNext::StepInto {
            moved = self.goto_first_child();
        }

        if !moved || goto_next == GotoNext::StepOver {
            // Otherwise try to move to the next sibling node.
            moved = self.goto_next_sibling();
            if !moved {
                // Otherwise try to move to the next ancestor sibling node.
                moved = self.goto_next_ancestor_sibling();
            }
        }

        if skip_extras {
            self.skip_extras();
        }

        moved
    }

    /// Move cursor to the next accessible node that has an error.
    pub fn goto_next_has_error(&mut self, mode: GotoNext, skip_extras: bool) -> bool {
        let mut moved;

        // Only descend if the current node has an error in the subtree.
        if self.node().has_error() {
            moved = self.goto_next(mode, skip_extras);
        } else {
            // Otherwise try to move to the next sibling node.
            moved = self.goto_next_sibling();
            if !moved {
                moved = self.goto_next_ancestor_sibling();
            }
        }

        moved
    }

    /// Move the cursor to the next ancestor sibling node.
    pub fn goto_next_ancestor_sibling(&mut self) -> bool {
        let mut moved;
        let mut finished = true;

        // Otherwise continue to ascend to parent nodes...
        loop {
            moved = self.goto_parent();
            if moved {
                // ... until we can move to a sibling node.
                if self.goto_next_sibling() {
                    finished = false;
                    break;
                }
            } else {
                break;
            }
        }

        self.done = finished;

        moved
    }

    /// Move the cursor to the parent node.
    pub fn goto_parent(&mut self) -> bool {
        let mut moved = false;

        if let Some(node) = self.node().parent() {
            self.node = node;
            moved = true;
        }

        moved
    }

    // FIXME
    fn skip_extras(&mut self) -> bool {
        #[rustfmt::skip]
        let extras: &[u16] = match self.language() {
            Language::DDlogDat => &[dat::kind::COMMENT_LINE],
            Language::DDlogDl  => &[ dl::kind::COMMENT_LINE, dl::kind::COMMENT_BLOCK],
        };
        let mut moved = false;
        loop {
            if !extras.contains(&self.node().kind_id()) {
                break;
            }
            moved = self.goto_next(GotoNext::StepOver, false);
        }
        moved
    }

    /// Return the current node's kind id.
    pub fn kind(&self) -> u16 {
        self.node().kind_id()
    }

    #[allow(missing_docs)]
    pub fn language(&self) -> Language {
        self.language
    }

    /// Return the current node for the cursor.
    pub fn node(&self) -> tree_sitter::Node<'tree> {
        self.node
    }

    #[allow(missing_docs)]
    pub fn reset(&mut self, node: tree_sitter::Node<'tree>) {
        self.node = node;
    }

    #[allow(missing_docs)]
    pub fn step(
        &mut self,
        want_kind: u16,
        node_move: NodeMove,
        goto_next: GotoNext,
    ) -> Result<tree_sitter::Node<'tree>, SyntaxError<()>> {
        let prev_node = self.node();

        // Conditionally move to the next node in the syntax tree; if a move was
        // attempted, record success or failure.
        let node_move = match node_move {
            NodeMove::Init => {
                self.skip_extras();
                true
            },
            NodeMove::Step => self.goto_next(goto_next, true),
        };

        // If a move was attempted and failed, return an error.
        if !node_move {
            let language = self.language();
            let range = self.node().range();
            let data = ();
            let error = SyntaxError::walker_move_error(language, range, data);
            return Err(error);
        }

        // Collect info on the destination node.
        let dest_node = self.node();
        let dest_kind = dest_node.kind_id();

        // If the destination node is a tree-sitter MISSING node, return an error.
        if dest_node.is_missing() {
            let language = self.language();
            let range = self.node().range();
            let data = ();
            let error = SyntaxError::node_missing_error(language, range, want_kind, data);
            self.reset(prev_node);
            return Err(error);
        }

        // If the destination node kind is not the expected node kind, return an error.
        if dest_kind != want_kind {
            let language = self.language();
            let range = self.node().range();
            let data = ();
            let error = SyntaxError::node_mismatch_error(language, range, dest_kind, want_kind, data);
            return Err(error);
        }

        // Otherwise return the destination node.
        Ok(dest_node)
    }
}
