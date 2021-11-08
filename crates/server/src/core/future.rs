use futures::future::{BoxFuture, FutureExt, Shared};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug, Clone)]
    pub struct EagerFuture<T> {
        #[pin]
        pinned: Shared<BoxFuture<'static, T>>,
    }
}

impl<T> EagerFuture<T> {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = T> + Send + 'static,
        T: Clone,
    {
        let pinned = future.boxed().shared();
        Self { pinned }
    }
}

impl<'a, T: Clone + Send + 'static> EagerFuture<Option<Option<T>>> {
    pub fn flatten(self) -> EagerFuture<Option<T>>
    where
        Self: Send,
    {
        let pinned = async { self.await.flatten() }.boxed().shared();
        EagerFuture { pinned }
    }
}

impl<T: Clone> Future for EagerFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().pinned.poll(cx)
    }
}

pub trait EagerFutureExt: Future {
    fn eager(self) -> EagerFuture<Option<<Self as Future>::Output>>;
}

impl<T: Sized> EagerFutureExt for T
where
    T: Future + Send + 'static,
    <T as Future>::Output: Clone + Send,
{
    fn eager(self) -> EagerFuture<Option<<T as Future>::Output>> {
        let pinned = tokio::spawn(self).map(Result::ok).boxed().shared();
        EagerFuture { pinned }
    }
}
