use tokio_proto::multiplex::{ClientProto, ServerProto};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use codec::TarantoolCodec;
use std::io;
use rmpv::{Value, Utf8String};
use futures::{Future, Stream, Sink};
use futures::future;

pub struct TarantoolProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for TarantoolProto {
    type Request = Vec<u8>;
    type Response = Result<Value, Utf8String>;
    type Transport = Framed<T, TarantoolCodec>;
    type BindTransport = Box<Future<Item = Self::Transport,
        Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(TarantoolCodec {
            handshaked: false,
        });
        Box::new(transport.into_future()
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                match line {
                    Some(ref msg) => {
                        println!("CLIENT: received server greeting message");
                        let ret = transport.send((0, vec![]));
                        Box::new(ret) as Self::BindTransport
                    }
                    _ => {
                        println!("SERVER: client handshake INVALID");
                        let err = io::Error::new(io::ErrorKind::Other,
                                                 "invalid handshake");
                        let ret = future::err(err);
                        Box::new(ret) as Self::BindTransport
                    }
                }
            }))
    }
}
