use crate::core::future::EagerFuture;
use futures::future::Future;
use std::convert::TryInto;

pub struct Text {
    pub language: crate::core::Language,
    pub content: EagerFuture<Option<ropey::Rope>>,
}

impl Text {
    pub fn new(
        language_id: impl TryInto<crate::core::Language, Error = anyhow::Error>,
        text: impl Future<Output = impl AsRef<str>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }
}
