use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
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

impl<S> NewService for Validate<S>
    where S: NewService<Request = ActionType, Response = AsyncResponse, Error = io::Error>,
          <S::Instance as Service>::Future: 'static
{
    type Request = ActionType;
    type Response = AsyncResponse;
    type Error = io::Error;
    type Instance = Validate<S::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}
