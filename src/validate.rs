use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
use rmpv::{Value, Utf8String};
use action::Action;
use std::marker::PhantomData;

pub struct Validate<S, A> {
    pub inner: S,
    pub action: PhantomData<A>,
}

impl<S, A> Service for Validate<S, A>
    where S: Service<Request = A, Response = Result<Value, Utf8String>, Error = io::Error>,
          S::Future: 'static,
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

impl<S, A> NewService for Validate<S, A>
    where S: NewService<Request =A, Response = Result<Value, Utf8String>, Error = io::Error>,
          A: Action,
          <S::Instance as Service>::Future: 'static,

{
    type Request = A;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Instance = Validate<S::Instance, A>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner, action: Self::Request })
    }
}
