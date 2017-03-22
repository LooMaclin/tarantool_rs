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
use utils::{scramble, header, get_response};
use request_type_key::RequestTypeKey;
use std::io::{Error, ErrorKind};
use rmp::encode::write_u32;
use async_response::AsyncResponse;
use auth::Auth;
use std::borrow::Cow;

pub struct TarantoolProto<A> where A: Action {
    pub _phantom: PhantomData<A>,
}

impl<T: AsyncRead + AsyncWrite + 'static, A: 'static> ClientProto<T> for TarantoolProto<A>  where A: Action {
    type Request = A;
    type Response = AsyncResponse;
    type Transport = Framed<T, TarantoolCodec<A>>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(TarantoolCodec {
            _phantom: PhantomData,
            tarantool_handshake_received: false,
        });
        let handshake = transport.into_future()
            // If the transport errors out, we don't care about the transport
            // anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it is the handshake
                match line {
                    Some(ref msg) => {
                        println!("CLIENT: received server handshake");
                        let &(request_id, resp) = msg;
                        match resp {
                         AsyncResponse::Handshake(handshake_data) => {
                             Box::new(transport.send((0, Auth {
                                 username: Cow::from("test"),
                                 scramble: handshake_data,
                             }))) as Self::BindTransport
                         },
                         AsyncResponse::Normal(_) => {
                             println!("CLIENT: server handshake INVALID");
                             let err = io::Error::new(io::ErrorKind::Other, "initial buffer is't [u8; 128]");
                             Box::new(future::err(err)) as Self::BindTransport
                         }
                        }
                    }
                    _ => {
                        println!("CLIENT: server handshake INVALID");
                        let err = io::Error::new(io::ErrorKind::Other, "empty initial buffer");
                        Box::new(future::err(err)) as Self::BindTransport
                    }
                }
            });

        Box::new(handshake)
    }
}
