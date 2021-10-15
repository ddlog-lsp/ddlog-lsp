use crate::core::{self};
use lsp_text::{RopeExt, TextEdit};
use std::{convert::TryFrom, sync::Arc};

#[cfg(feature = "runtime-agnostic")]
use async_lock::Mutex;
#[cfg(feature = "runtime-tokio")]
use tokio::sync::Mutex;

pub struct Document {
    pub uri: lsp::Url,
    pub language: core::Language,
    pub content: ropey::Rope,
    pub parser: tree_sitter::Parser,
    pub tree: tree_sitter::Tree,
}

impl Document {
    pub fn open(params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<Option<Self>> {
        let uri = params.text_document.uri;
        let language = core::Language::try_from(params.text_document.language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let content = ropey::Rope::from(params.text_document.text);
        let result = {
            let content = content.clone();
            let byte_idx = 0;
            let text = content.chunks().collect::<String>();
            let old_tree = None;
            parser.parse(text, old_tree)?
        };
        Ok(result.map(|tree| core::Document {
            uri,
            language,
            content,
            parser,
            tree,
        }))
    }

    pub async fn change<'changes>(
        session: Arc<core::Session>,
        uri: &lsp::Url,
        content: &ropey::Rope,
        edits: &[TextEdit<'changes>],
    ) -> anyhow::Result<Option<tree_sitter::Tree>> {
        let result = {
            let parser = session.get_mut_parser(uri).await?;
            let mut parser = parser.lock().await;

            let text = content.chunks().collect::<String>();

            let old_tree = session.get_mut_tree(uri).await?;
            let mut old_tree = old_tree.lock().await;

            for edit in edits {
                old_tree.edit(&edit.input_edit);
            }

            parser.parse(text, Some(&*old_tree))?
        };

        if let Some(tree) = result {
            {
                let tree = tree.clone();
                *session.get_mut_tree(uri).await?.value_mut() = Mutex::new(tree);
            }
            Ok(Some(tree))
        } else {
            Ok(None)
        }
    }

    pub fn text(&self) -> core::Text {
        core::Text {
            language: self.language,
            content: self.content.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DocumentState {
    Closed,
    Opened,
}
