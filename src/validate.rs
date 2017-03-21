use std::io;
use futures::Future;
use tokio_service::{Service, NewService};
use rmpv::{Value, Utf8String};
use action::Action;

pub struct Validate<T, K> {
    pub inner: T,
}

impl<T, K> Service for Validate<T,K>
    where T: Service<Request = Vec<u8>, Response = Result<Value, Utf8String>, Error = io::Error>,
          T::Future: 'static,
         K: Action
{
    type Request = Vec<u8>;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Future = Box<Future<Item = Result<Value, Utf8String>, Error = io::Error>>;

    fn call<K>(&self, req: K) -> Self::Future where K: Action {
        Box::new(self.inner.call(req).and_then(|resp| Ok(resp)))
    }
}

impl<T, K> NewService for Validate<T, K>
    where T: NewService<Request = Vec<u8>, Response = Result<Value, Utf8String>, Error = io::Error>,
          K: Action,
          <T::Instance as Service>::Future: 'static,

{
    type Request = Vec<u8>;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;
    type Instance = Validate<T::Instance, K>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}
