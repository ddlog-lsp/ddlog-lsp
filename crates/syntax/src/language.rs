//! Functionality related to [`tree-sitter::Language`].
use crate::node::NodeWalker;

#[allow(missing_docs)]
pub trait HasWalker<'tree> {
    fn walker(&mut self) -> &mut NodeWalker<'tree>;
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum NodeMove {
    Init,
    Step,
}

#[macro_use]
pub mod utils;

/// Functions for working with the `.dat` grammar.
pub mod dat;

/// Functions for working with the `.dl` grammar.
pub mod dl;
