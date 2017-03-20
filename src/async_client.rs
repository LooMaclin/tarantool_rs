use validate::Validate;
use tokio_proto::multiplex::ClientService;
use tokio_core::net::TcpStream;
use proto::TarantoolProto;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;
use futures::Future;
use std::io;
use tokio_proto::TcpClient;
use tokio_service::Service;
use rmpv::{Value, Utf8String};
use action::Action;
use utils::build_auth_body;
use state::State;
use std::borrow::Cow;
use greeting_packet::GreetingPacket;
use utils::{header, build_request, process_response};
use std::str::FromStr;
use insert::Insert;

pub struct AsyncClient<T> {
    data: T,
    inner: Validate<ClientService<TcpStream, TarantoolProto>>,
}

impl <T> AsyncClient<T>{
    pub fn auth<'a, S>(address: S, user: S, password: S, handle: &Handle) -> Box<Future<Item = AsyncClient<T>, Error = io::Error>>
    where S: Into<Cow<'a, str>> {
        let addr = SocketAddr::from_str(address.into().as_ref()).unwrap();
        let ret = TcpClient::new(TarantoolProto)
            .connect(&addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service};
                AsyncClient { inner: validate,  data: Insert {
                    space: 512,
                    keys: vec![],
                }}
            });
        Box::new(ret)
    }
}

impl<T> Service for AsyncClient<T>
    where T: Service,
          T::Request: Action {
    type Request = T::Request;
    type Response = Result<Value, Utf8String>;
    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req)
    }
}
