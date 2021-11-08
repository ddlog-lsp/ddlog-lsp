use futures::future::{BoxFuture, FutureExt, Shared};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug, Clone)]
    pub struct EagerFuture<'a, T> {
        #[pin]
        pinned: Shared<BoxFuture<'a, T>>,
    }
}

impl<'a, T> EagerFuture<'a, T> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'a,
        <F as Future>::Output: Clone,
    {
        let pinned = future.boxed().shared();
        Self { pinned }
    }
}

impl<'a, T: Clone> Future for EagerFuture<'a, T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().pinned.poll(cx)
    }
}

pub trait EagerFutureExt: Sized {
    fn eager(self) -> EagerFuture<'static, Option<<Self as Future>::Output>>
    where
        Self: Future + Send + 'static,
        <Self as Future>::Output: Clone + Send + 'static,
    {
        let pinned = tokio::spawn(self).map(Result::ok).boxed().shared();
        EagerFuture { pinned }
    }
}
