use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
use rmpv::{Value, Utf8String};
use action::Action;

pub struct Validate<T, K> {
    pub inner: T,
}

impl<T, A> Service for Validate<T, A>
    where T: Service<Request =A, Response = Result<Value, Utf8String>, Error = io::Error>,
          T::Future: 'static,
          A: Action
{
    type Request = A;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Future = Box<Future<Item = Result<Value, Utf8String>, Error = io::Error>>;

    fn call(&self, req: Self::Response) -> Self::Future {
        Box::new(self.inner.call(req).and_then(|resp| Ok(resp)))
    }
}

impl<T, A> NewService for Validate<T, A>
    where T: NewService<Request =A, Response = Result<Value, Utf8String>, Error = io::Error>,
          A: Action,
          <T::Instance as Service>::Future: 'static,

{
    type Request = A;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Instance = Validate<T::Instance, A>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}
