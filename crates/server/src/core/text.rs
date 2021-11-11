use crate::core::future::EagerFuture;
use futures::future::Future;
use std::convert::TryInto;

#[derive(Clone)]
pub struct Text {
    pub language: crate::core::Language,
    content: EagerFuture<Option<ropey::Rope>>,
}

impl Text {
    pub fn new(language: crate::core::Language, content: EagerFuture<Option<ropey::Rope>>) -> Self {
        Self { language, content }
    }

    pub async fn get_content(&self) -> anyhow::Result<ropey::Rope> {
        self.content
            .clone()
            .await
            .ok_or_else(|| anyhow::anyhow!("could not resolve text content"))
    }

    pub fn set_content(&mut self, content: EagerFuture<Option<ropey::Rope>>) {
        self.content = content;
    }
}
