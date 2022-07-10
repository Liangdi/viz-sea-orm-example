use std::{
    convert::Infallible,
    future::{ready, Ready},
    sync::Arc,
    task::{Context, Poll},
};

use super::Stream;
use crate::{Router, Tree};

#[derive(Clone)]
pub struct ServiceMaker {
    pub(crate) tree: Arc<Tree>,
}

impl ServiceMaker {
    pub fn new(router: Router) -> Self {
        Self {
            tree: Arc::new(router.into()),
        }
    }
}

impl From<Router> for ServiceMaker {
    fn from(router: Router) -> Self {
        Self::new(router)
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
impl hyper::service::Service<&tokio::net::TcpStream> for ServiceMaker {
    type Response = Stream;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, socket: &tokio::net::TcpStream) -> Self::Future {
        if let Err(_) = socket.set_nodelay(true) {
            // TODO: trace error
        }
        ready(Ok(Stream::new(self.tree.clone(), socket.peer_addr().ok())))
    }
}

#[cfg(all(unix, feature = "unix-socket"))]
impl hyper::service::Service<&tokio::net::UnixStream> for ServiceMaker {
    type Response = Stream;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: &tokio::net::UnixStream) -> Self::Future {
        ready(Ok(Stream::new(self.tree.clone(), None)))
    }
}