use tokio_proto::multiplex::{ClientProto, ServerProto};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use codec::TarantoolCodec;
use std::io;
use rmpv::{Value, Utf8String};
use futures::{Future, Stream, Sink};
use futures::future;
use action::Action;
use std::marker::PhantomData;
use greeting_packet::GreetingPacket;
use utils::{scramble, header, build_auth_body, get_response};
use request_type_key::RequestTypeKey;
use std::io::{Error, ErrorKind};
use rmp::encode::write_u32;

pub struct TarantoolProto<A> where A: Action {
    pub _phantom: PhantomData<A>,
}

impl<T: AsyncRead + AsyncWrite + 'static, A: 'static> ClientProto<T> for TarantoolProto<A>  where A: Action {
    type Request = A;
    type Response = Result<Value, Utf8String>;
    type Transport = Framed<T, TarantoolCodec<A>>;
    type BindTransport = Box<Future<Item = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(TarantoolCodec {
            _phantom: PhantomData,
            handshaked: false,
        });
        // The handshake requires that the client sends `You ready?`,
        // so wait to receive that line. If anything else is sent,
        // error out the connection
        Box::new(transport.into_future()
            // If the transport errors out, we don't care about
            // the transport anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it
                // is the handshake
                match line {
                    Some(ref msg) => {
                        println!("SERVER: received client handshake");
                        Box::new(ret) as Self::BindTransport
                    }
                    _ => {
                        // The client sent an unexpected handshake,
                        // error out the connection
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
