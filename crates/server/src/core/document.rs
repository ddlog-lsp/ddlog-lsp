use crate::core::future::EagerFuture;
use futures::{future, FutureExt};
use lsp_text::{RopeExt, TextEdit};
use std::{convert::TryFrom, sync::Arc};

#[cfg(feature = "runtime-agnostic")]
use async_lock::Mutex;
#[cfg(feature = "runtime-tokio")]
use tokio::sync::Mutex;

pub struct DocumentFuture {
    pub uri: lsp::Url,
    pub language: crate::core::Language,
    pub content: EagerFuture<'static, ropey::Rope>,
    pub parser: EagerFuture<'static, Option<Arc<Mutex<tree_sitter::Parser>>>>,
    pub tree: EagerFuture<'static, Option<tree_sitter::Tree>>,
}

impl DocumentFuture {
    pub fn open_from_lsp(params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<Self> {
        let uri = params.text_document.uri;
        let language = crate::core::Language::try_from(params.text_document.language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let content = ropey::Rope::from(params.text_document.text);

        let tree = {
            let content = content.clone();
            let byte_idx = 0;
            let text = content.chunks().collect::<String>();
            let old_tree = None;
            parser.parse(text, old_tree)?
        };

        let content = EagerFuture::new(future::ready(content));
        let parser = EagerFuture::new(future::ready(Some(Arc::new(Mutex::new(parser)))));
        let tree = EagerFuture::new(future::ready(tree));

        Ok(DocumentFuture {
            uri,
            language,
            content,
            parser,
            tree,
        })
    }

    pub async fn open_from_uri(uri: lsp::Url) -> anyhow::Result<Self> {
        if let Ok(path) = uri.to_file_path() {
            let language = crate::core::Language::try_from(path.as_path())?;

            let mut parser = {
                let language = language.clone();
                let val = tokio::spawn(async move {
                    let val = tree_sitter::Parser::try_from(language);
                    if let Ok(parser) = val {
                        Some(Arc::new(Mutex::new(parser)))
                    } else {
                        None
                    }
                });
                let val = val.map(Result::ok).map(Option::flatten);
                let val = EagerFuture::new(val);
                val
            };

            let text = tokio::fs::read_to_string(path).await?;
            let content = ropey::Rope::from(text);

            let tree = {
                let parser = parser.clone();
                let content = content.clone();
                let byte_idx = 0;
                let text = content.chunks().collect::<String>();
                let old_tree = None;
                let val = tokio::spawn(async move {
                    if let Some(parser) = parser.await {
                        let mut parser = parser.lock().await;
                        parser.parse(text, old_tree).unwrap()
                    } else {
                        None
                    }
                });
                let val = val.map(Result::ok).map(Option::flatten);
                EagerFuture::new(val)
            };

            let content = {
                let val = ropey::Rope::from(content);
                let val = future::ready(val);
                EagerFuture::new(val)
            };

            Ok(DocumentFuture {
                uri,
                language,
                content,
                parser,
                tree,
            })
        } else {
            anyhow::bail!("Could not convert uri to file path: {:#?}", uri);
        }
    }

    pub async fn change<'changes>(
        session: Arc<crate::core::Session>,
        uri: &lsp::Url,
        content: &ropey::Rope,
        edits: &[TextEdit<'changes>],
    ) -> anyhow::Result<Option<tree_sitter::Tree>> {
        todo!()
    }

    pub fn text(&self) -> crate::core::Text {
        todo!()
    }
}

pub struct Document {
    pub uri: lsp::Url,
    pub language: crate::core::Language,
    pub content: ropey::Rope,
    pub parser: tree_sitter::Parser,
    pub tree: tree_sitter::Tree,
}

impl Document {
    pub fn open_from_lsp(params: lsp::DidOpenTextDocumentParams) -> anyhow::Result<Option<Self>> {
        let uri = params.text_document.uri;
        let language = crate::core::Language::try_from(params.text_document.language_id.as_str())?;
        let mut parser = tree_sitter::Parser::try_from(language)?;
        let content = ropey::Rope::from(params.text_document.text);
        let result = {
            let content = content.clone();
            let byte_idx = 0;
            let text = content.chunks().collect::<String>();
            let old_tree = None;
            parser.parse(text, old_tree)?
        };
        Ok(result.map(|tree| Document {
            uri,
            language,
            content,
            parser,
            tree,
        }))
    }

    pub async fn open_from_uri(uri: lsp::Url) -> anyhow::Result<Option<Self>> {
        if let Ok(path) = uri.to_file_path() {
            let language = crate::core::Language::try_from(path.as_path())?;
            let params = lsp::DidOpenTextDocumentParams {
                text_document: {
                    let language_id = language.id().into();
                    let version = Default::default();
                    let text = tokio::fs::read_to_string(path).await?;
                    lsp::TextDocumentItem {
                        uri,
                        language_id,
                        version,
                        text,
                    }
                },
            };
            Self::open_from_lsp(params)
        } else {
            anyhow::bail!("Could not convert uri to file path: {:#?}", uri);
        }
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

    pub fn text(&self) -> crate::core::Text {
        crate::core::Text {
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
