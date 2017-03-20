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

pub struct AsyncClient<'a> {
    state: State<'a>,
    inner: Validate<ClientService<TcpStream, TarantoolProto>>,
}

impl<'a> AsyncClient<'a> {
    pub fn auth<S>(address: S, user: S, password: S, handle: &Handle) -> Box<Future<Item = AsyncClient, Error = io::Error>>
    where S: Into<Cow<'a, str>> {
        let addr = SocketAddr::from_str(address.into().as_ref()).unwrap();
        let ret = TcpClient::new(TarantoolProto)
            .connect(&addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service};
                AsyncClient { inner: validate,
                    state:
                State {
                    address: "abc".into(),
                    user: "abc".into(),
                    password: "abc".into(),
                    greeting_packet: GreetingPacket::new("", ""),
                    request_id: 0,
                } }
            });
        Box::new(ret)
    }

    pub fn request<I>(&mut self, request_body: &I) -> Box<Future<Item = Value, Error = Utf8String>>
        where I: Action
    {
        let request = build_request(request_body, self.state.get_id());
        let response = self.call(request);
        response
    }
}

impl<'a> Service for AsyncClient<'a> {
    type Request = Vec<u8>;
    type Response = Value;
    type Error = Utf8String;
    // For simplicity, box the future.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.call(req)
    }
}
