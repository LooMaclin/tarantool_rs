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
use action_type::ActionType;
use hex_slice::AsHex;

#[derive(Debug)]
pub struct TarantoolProto;

impl<T> ClientProto<T> for TarantoolProto
    where T: AsyncRead + AsyncWrite + 'static
{
    type Request = ActionType;
    type Response = AsyncResponse;
    type Transport = Framed<T, TarantoolCodec>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(TarantoolCodec {
            tarantool_handshake_received: false,
        });
        let handshake = transport.into_future()
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                match line {
                    Some(ref msg) => {
                        println!("CLIENT: received server handshake");
                        let &(request_id, ref resp) = msg;
                        match resp {
                            &AsyncResponse::Handshake(ref handshake_data) => {
                                println!("Handshaked data: {:#X}", handshake_data.as_hex());
                                Box::new(transport.send((0,
                                                         ActionType::Auth(Auth {
                                    username: String::from("test"),
                                    scramble: handshake_data.clone(),
                                })))) as Self::BindTransport
                            }
                            &AsyncResponse::Normal(_) => {
                                println!("CLIENT: server handshake INVALID");
                                let err = io::Error::new(io::ErrorKind::Other,
                                                         "initial buffer is't [u8; 128]");
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
