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
use std::borrow::Cow;
use std::str::FromStr;
use async_response::AsyncResponse;
use action_type::ActionType;

#[derive(Debug)]
pub struct AsyncClient {
    pub inner: Validate<ClientService<TcpStream, TarantoolProto>>,
}

impl AsyncClient {
    pub fn auth<'a, S>(address: S,
                       user: S,
                       password: S,
                       handle: &Handle)
                       -> Box<Future<Item = AsyncClient, Error = io::Error>>
        where S: Into<Cow<'a, str>>
    {
        let addr = SocketAddr::from_str(address.into().as_ref()).unwrap();
        let ret = TcpClient::new(TarantoolProto {
                user: user.into().into_owned(),
                password: password.into().into_owned(),
            })
            .connect(&addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service };
                AsyncClient { inner: validate }
            });
        Box::new(ret)
    }
}

impl Service for AsyncClient {
    type Request = ActionType;
    type Response = AsyncResponse;
    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req)
    }
}
