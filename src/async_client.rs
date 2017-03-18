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


pub struct AsyncClient {
    inner: Validate<ClientService<TcpStream, TarantoolProto>>,
}

impl AsyncClient {

    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item =AsyncClient, Error = io::Error>> {
        let ret = TcpClient::new(TarantoolProto)
            .connect(addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service};
                AsyncClient { inner: validate }
            });

        Box::new(ret)
    }
}

impl Service for AsyncClient {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    // For simplicity, box the future.
    type Future = Box<Future<Item = Vec<u8>, Error = io::Error>>;

    fn call(&self, req: Vec<u8>) -> Self::Future {
        self.inner.call(req)
    }
}