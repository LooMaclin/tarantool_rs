use std::io;
use futures::Future;
use tokio_service::{Service};
use action_type::ActionType;
use async_response::AsyncResponse;

#[derive(Debug)]
pub struct Validate<S> {
    pub inner: S,
}

impl<S> Service for Validate<S>
    where S: Service<Request = ActionType, Response = AsyncResponse, Error = io::Error>,
          S::Future: 'static
{
    type Request = ActionType;
    type Response = AsyncResponse;
    type Error = io::Error;
    type Future = Box<Future<Item = AsyncResponse, Error = io::Error>>;

    fn call(&self, req: ActionType) -> Self::Future {
        Box::new(self.inner.call(req).and_then(|resp| Ok(resp)))
    }
}