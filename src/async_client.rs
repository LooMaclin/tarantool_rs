extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;

use futures::{future, Future};
use tokio_core::io::{Io, Codec, EasyBuf, Framed};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::{TcpClient, TcpServer};
use tokio_proto::multiplex::{RequestId, ServerProto, ClientProto, ClientService};
use tokio_service::{Service, NewService};
use byteorder::{BigEndian, ByteOrder};
use std::{io, str};
use std::net::SocketAddr;

pub struct TarantoolAsyncClient {
    inner: Validate<ClientService<TcpStream, TarantoolProto>>,
}
struct Validate<T> {
    inner: T,
}

struct TarantoolCodec;

struct TarantoolProto;

impl TarantoolAsyncClient {
    pub fn connect(addr: &SocketAddr,
                   handle: &Handle)
                   -> Box<Future<Item = TarantoolAsyncClient, Error = io::Error>> {
        let ret = TcpClient::new(TarantoolProto)
            .connect(addr, handle)
            .map(|client_service| {
                let validate = Validate { inner: client_service };
                TarantoolAsyncClient { inner: validate }
            });

        Box::new(ret)
    }
}

impl Service for TarantoolAsyncClient {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Vec<u8>, Error = io::Error>>;

    fn call(&self, req: Vec<u8>) -> Self::Future {
        self.inner.call(req)
    }
}

impl<T> Service for Validate<T>
    where T: Service<Request = Vec<u8>, Response = Vec<u8>, Error = io::Error>,
          T::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Future = Box<Future<Item = Vec<u8>, Error = io::Error>>;

    fn call(&self, req: Vec<u8>) -> Self::Future {
        Box::new(self.inner
            .call(req)
            .and_then(|resp| Ok(resp)))
    }
}

impl<T> NewService for Validate<T>
    where T: NewService<Request = Vec<u8>, Response = Vec<u8>, Error = io::Error>,
          <T::Instance as Service>::Future: 'static
{
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    type Error = io::Error;
    type Instance = Validate<T::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let inner = try!(self.inner.new_service());
        Ok(Validate { inner: inner })
    }
}

impl Codec for TarantoolCodec {
    type In = (RequestId, Vec<u8>);
    type Out = (RequestId, Vec<u8>);

    fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<(RequestId, Vec<u8>)>, io::Error> {
        if buf.len() == 128 {
            return Ok(Some((1 as RequestId, vec![1, 2, 3])));
        }
        // if let Some(n) = buf.as_ref()[4..].iter().position(|b| *b == b'\n') {
        // let line = buf.drain_to(n + 4);
        // buf.drain_to(1);
        // let request_id = BigEndian::read_u32(&line.as_ref()[0..4]);
        // return match str::from_utf8(&line.as_ref()[4..]) {
        // Ok(s) => Ok(Some((request_id as RequestId, s.to_string()))),
        // Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
        // }
        // }
        Ok(None)
    }

    fn encode(&mut self, msg: (RequestId, Vec<u8>), buf: &mut Vec<u8>) -> io::Result<()> {
        let (request_id, msg) = msg;

        let mut encoded_request_id = [0; 4];
        BigEndian::write_u32(&mut encoded_request_id, request_id as u32);

        buf.extend(&encoded_request_id);
        buf.extend(&msg[..]);
        buf.push(b'\n');
        Ok(())
    }
}

impl<T: Io + 'static> ClientProto<T> for TarantoolProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    /// `Framed<T, TarantoolCodec>` is the return value of `io.framed(TarantoolCodec)`
    type Transport = Framed<T, TarantoolCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TarantoolCodec))
    }
}

impl<T: Io + 'static> ServerProto<T> for TarantoolProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;
    /// `Framed<T, TarantoolCodec>` is the return value of `io.framed(TarantoolCodec)`
    type Transport = Framed<T, TarantoolCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TarantoolCodec))
    }
}
