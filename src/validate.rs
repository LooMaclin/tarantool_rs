use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
use rmpv::{Value, Utf8String};

pub struct Validate<T> {
    pub inner: T,
}

impl<T> Service for Validate<T>
    where T: Service<Request = Vec<u8>, Response = Value, Error = Utf8String>,
          T::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Value;
    type Error = Utf8String;
    type Future = Box<Future<Item = Value, Error = Utf8String>>;

    fn call(&self, req: Vec<u8>) -> Self::Future {
        Box::new(self.inner.call(req).and_then(|resp| Ok(resp)))
    }
}

impl<T> NewService for Validate<T>
    where T: NewService<Request = Vec<u8>, Response = Value, Error = Utf8String>,
          <T::Instance as Service>::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Value;
    type Error = Utf8String;
    type Instance = Validate<T::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}
