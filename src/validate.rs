use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
use rmpv::{Value, Utf8String};

pub struct Validate<T> {
    pub inner: T,
}

impl<T> Service for Validate<T>
    where T: Service<Request = Vec<u8>, Response = Result<Value, Utf8String>, Error = io::Error>,
          T::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Future = Box<Future<Item = Result<Value, Utf8String>, Error = io::Error>>;

    fn call(&self, req: Vec<u8>) -> Self::Future {
        Box::new(self.inner.call(req).and_then(|resp| Ok(resp)))
    }
}

impl<T> NewService for Validate<T>
    where T: NewService<Request = Vec<u8>, Response = Result<Value, Utf8String>, Error = io::Error>,
          <T::Instance as Service>::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Instance = Validate<T::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}
