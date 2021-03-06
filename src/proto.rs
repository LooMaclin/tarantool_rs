use tokio_proto::multiplex::ClientProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use codec::TarantoolCodec;
use std::io;
use futures::{Future, Stream, Sink};
use futures::future;
use async_response::AsyncResponse;
use auth::Auth;
use action_type::ActionType;
use hex_slice::AsHex;

#[derive(Debug)]
pub struct TarantoolProto {
    pub user: String,
    pub password: String,
}

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
            tarantool_auth_message_received: false,
        });
        let handshake = transport.into_future()
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| match line {
                Some(ref msg) => {
                    debug!("CLIENT: received server handshake");
                    let &(request_id, ref resp) = msg;
                    match resp {
                        &AsyncResponse::Handshake(ref handshake_data) => {
                            debug!("Handshaked data: {:#X}", handshake_data.as_hex());
                            Box::new(transport.send((request_id,
                                                     ActionType::Auth(Auth {
                                username: String::from("test"),
                                scramble: handshake_data.clone(),
                            })))) as Self::BindTransport
                        }
                        &AsyncResponse::Normal(_) => {
                            debug!("CLIENT: server handshake INVALID");
                            let err = io::Error::new(io::ErrorKind::Other,
                                                     "initial buffer is't [u8; 128]");
                            Box::new(future::err(err)) as Self::BindTransport
                        }
                    }
                }
                _ => {
                    debug!("CLIENT: server handshake INVALID");
                    let err = io::Error::new(io::ErrorKind::Other, "empty initial buffer");
                    Box::new(future::err(err)) as Self::BindTransport
                }
            });
        Box::new(handshake)
    }
}
