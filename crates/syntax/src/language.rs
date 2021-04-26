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
pub mod utils {
    #![allow(missing_docs)]
    #![allow(unused)]

    use crate::node::GotoNext;

    ddlog_lsp_macros::impl_choice!(1);
    ddlog_lsp_macros::impl_choice!(2);
    ddlog_lsp_macros::impl_choice!(3);
    ddlog_lsp_macros::impl_choice!(4);
    ddlog_lsp_macros::impl_choice!(5);
    ddlog_lsp_macros::impl_choice!(6);
    ddlog_lsp_macros::impl_choice!(7);
    ddlog_lsp_macros::impl_choice!(9);
    ddlog_lsp_macros::impl_choice!(11);
    ddlog_lsp_macros::impl_choice!(12);
    ddlog_lsp_macros::impl_choice!(14);
    ddlog_lsp_macros::impl_choice!(50);
    ddlog_lsp_macros::impl_choice!(51);

    ddlog_lsp_macros::impl_seq!(2);
    ddlog_lsp_macros::impl_seq!(3);
    ddlog_lsp_macros::impl_seq!(4);
    ddlog_lsp_macros::impl_seq!(5);
    ddlog_lsp_macros::impl_seq!(6);
    ddlog_lsp_macros::impl_seq!(7);
    ddlog_lsp_macros::impl_seq!(8);
    ddlog_lsp_macros::impl_seq!(9);
    ddlog_lsp_macros::impl_seq!(10);
    ddlog_lsp_macros::impl_seq!(11);

    use crate::{
        error::SyntaxError,
        language::{HasWalker, NodeMove},
    };

    pub trait Choice<'tree, Vis>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        fn choice(&self, visitor: &mut Vis, m: NodeMove) -> Result<(), SyntaxError<()>>;
    }

    pub fn choice<'tree, Vis, T>(funs: T) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
        T: Choice<'tree, Vis>,
    {
        move |visitor, node_move| {
            // log::info!("choice");
            funs.choice(visitor, node_move)
        }
    }

    pub fn optional<'tree, Vis>(
        fun: impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>,
    ) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        move |visitor, node_move| {
            // log::info!("optional");
            let prev = visitor.walker().node();
            if fun(visitor, node_move).is_err() {
                visitor.walker().reset(prev);
            }
            Ok(())
        }
    }

    pub mod repeat {
        use crate::{
            error::SyntaxError,
            language::{HasWalker, NodeMove},
        };

        pub fn eof<'tree, Vis>(
            fun: impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>,
        ) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
        where
            Vis: HasWalker<'tree> + ?Sized,
        {
            move |visitor, node_move| {
                loop {
                    // log::info!("repeat::eof");
                    let prev = visitor.walker().node();
                    match fun(visitor, node_move) {
                        Ok(_) => {
                            continue;
                        },
                        err @ Err(_) => {
                            if visitor.walker().done {
                                return Ok(());
                            } else {
                                visitor.walker().reset(prev);
                                return err;
                            }
                        },
                    }
                }
                Ok(())
            }
        }
    }

    pub fn repeat<'tree, Vis>(
        fun: impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>,
    ) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        move |visitor, node_move| {
            loop {
                // log::info!("repeat");
                let prev = visitor.walker().node();
                if visitor.walker().done {
                    break;
                }
                match fun(visitor, node_move) {
                    Ok(_) => {
                        continue;
                    },
                    err @ Err(_) => {
                        visitor.walker().reset(prev);
                        break;
                    },
                }
            }
            Ok(())
        }
    }

    pub fn repeat1<'tree, Vis>(
        fun: impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>,
    ) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        move |visitor, node_move| {
            let mut succeeded_once = false;
            if visitor.walker().done {
                let language = visitor.walker().language();
                let range = visitor.walker().node().range();
                let data = ();
                let error = SyntaxError::walker_done_early(language, range, data);
                return Err(error);
            }
            loop {
                // log::info!("repeat1");
                let prev = visitor.walker().node();
                match fun(visitor, node_move) {
                    Ok(_) => {
                        succeeded_once = true;
                    },
                    err @ Err(_) => {
                        visitor.walker().reset(prev);
                        if succeeded_once {
                            break;
                        } else {
                            return err;
                        }
                    },
                }
            }
            Ok(())
        }
    }

    pub trait Seq<'tree, Vis>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        fn seq(&self, visitor: &mut Vis, m: NodeMove) -> Result<(), SyntaxError<()>>;
    }

    pub fn seq<'tree, Vis, T>(funs: T) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
        T: Seq<'tree, Vis>,
    {
        move |visitor, node_move| {
            // log::info!("seq");
            funs.seq(visitor, node_move)
        }
    }

    pub fn restore<'tree, Vis>(
        fun: impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>,
    ) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        move |visitor, node_move| {
            // log::info!("restore");
            let prev = visitor.walker().node();
            match fun(visitor, node_move) {
                Ok(result) => Ok(result),
                err @ Err(_) => {
                    visitor.walker().reset(prev);
                    err
                },
            }
        }
    }

    pub fn token<'tree, Vis>(token: u16) -> impl Fn(&mut Vis, NodeMove) -> Result<(), SyntaxError<()>>
    where
        Vis: HasWalker<'tree> + ?Sized,
    {
        move |visitor, node_move| {
            visitor.walker().step(token, node_move, GotoNext::StepInto)?;
            Ok(())
        }
    }
}

/// Functions for working with the `.dat` grammar.
pub mod dat;

/// Functions for working with the `.dl` grammar.
pub mod dl;
