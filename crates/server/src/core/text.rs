use crate::core::future::EagerFuture;
use futures::future::Future;
use std::convert::TryInto;

pub struct Text {
    pub language: crate::core::Language,
    pub content: EagerFuture<Option<ropey::Rope>>,
}
