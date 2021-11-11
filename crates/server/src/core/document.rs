use crate::core::future::{EagerFuture, EagerFutureExt};
use futures::{future, FutureExt, TryFutureExt};
use lsp_text::{RopeExt, TextEdit};
use std::{convert::TryFrom, sync::Arc};

#[cfg(feature = "runtime-agnostic")]
use async_lock::Mutex;
#[cfg(feature = "runtime-tokio")]
use tokio::sync::Mutex;

pub struct Document {
    pub uri: lsp::Url,
    pub language: crate::core::Language,
    pub content: EagerFuture<Option<ropey::Rope>>,
    pub parser: Arc<Mutex<tree_sitter::Parser>>,
    pub tree: EagerFuture<Option<Arc<Mutex<tree_sitter::Tree>>>>,
}

impl Document {
    pub fn open_from_lsp(params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<Self> {
        let uri = params.text_document.uri;
        let language = crate::core::Language::try_from(params.text_document.language_id.as_str())?;

        let parser = Arc::new(Mutex::new(tree_sitter::Parser::try_from(language)?));

        let content = ropey::Rope::from(params.text_document.text);

        let tree = {
            let parser = parser.clone();
            let content = content.clone();
            let byte_idx = 0;
            let old_tree = None;

            async move {
                let text = content.chunks().collect::<String>();
                let mut parser = parser.lock().await;
                parser
                    .parse(text, old_tree)
                    .ok()
                    .flatten()
                    .map(|tree| Arc::new(Mutex::new(tree)))
            }
        }
        .eager()
        .flatten();

        let content = EagerFuture::new(future::ready(Some(content)));

        Ok(Document {
            uri,
            language,
            content,
            parser,
            tree,
        })
    }

    pub fn open_from_uri(uri: lsp::Url) -> anyhow::Result<Self> {
        let path = uri
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("Could not convert uri to file path: {:#?}", uri))?;

        let language = crate::core::Language::try_from(path.as_path())?;

        let parser = Arc::new(Mutex::new(tree_sitter::Parser::try_from(language)?));

        let content = tokio::fs::read_to_string(path)
            .map_ok(ropey::Rope::from)
            .map(Result::ok)
            .eager()
            .flatten();

        let tree = {
            let parser = parser.clone();
            let content = content.clone();
            let byte_idx = 0;
            let old_tree = None;

            async move {
                if let Some(text) = content.await {
                    let text = text.chunks().collect::<String>();
                    let mut parser = parser.lock().await;
                    parser
                        .parse(text, old_tree)
                        .ok()
                        .flatten()
                        .map(|tree| Arc::new(Mutex::new(tree)))
                } else {
                    None
                }
            }
        }
        .eager()
        .flatten();

        Ok(Document {
            uri,
            language,
            content,
            parser,
            tree,
        })
    }

    pub async fn change<'changes>(
        session: Arc<crate::core::Session>,
        uri: &lsp::Url,
        content: &ropey::Rope,
        edits: &[TextEdit<'changes>],
    ) -> anyhow::Result<Option<tree_sitter::Tree>> {
        let result = {
            let parser = session.get_mut_parser(uri).await?;
            let mut parser = parser.lock().await;

            let text = content.chunks().collect::<String>();

            let old_tree = session
                .get_mut_tree(uri)
                .await?
                .clone()
                .await
                .ok_or_else(|| anyhow::anyhow!("could not resolve previous tree for uri: {:#?}", uri))?;

            let mut old_tree = old_tree.lock().await;

            for edit in edits {
                old_tree.edit(&edit.input_edit);
            }

            parser.parse(text, Some(&*old_tree))?
        };

        if let Some(tree) = result {
            {
                let tree = future::ready(Arc::new(Mutex::new(tree.clone()))).eager();
                *session.get_mut_tree(uri).await?.value_mut() = tree;
            }
            Ok(Some(tree))
        } else {
            Ok(None)
        }
    }

    pub fn text(&self) -> crate::core::Text {
        crate::core::Text::new(self.language, self.content.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DocumentState {
    Closed,
    Opened,
}
