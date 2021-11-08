use crate::core::future::EagerFuture;
use futures::future::Future;
use std::convert::TryInto;

pub struct TextFuture {
    pub language: crate::core::Language,
    pub content: EagerFuture<ropey::Rope>,
}

impl TextFuture {
    pub fn new(
        language_id: impl TryInto<crate::core::Language, Error = anyhow::Error>,
        text: impl Future<Output = impl AsRef<str>>,
    ) -> anyhow::Result<Self> {
        todo!()
    }
}

pub struct Text {
    pub language: crate::core::Language,
    pub content: ropey::Rope,
}

impl Text {
    pub fn new(
        language_id: impl TryInto<crate::core::Language, Error = anyhow::Error>,
        text: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        let text = text.as_ref();
        let language = language_id.try_into()?;
        let content = ropey::Rope::from_str(text);
        Ok(Text { language, content })
    }
}
