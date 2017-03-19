use tokio_proto::multiplex::{ClientProto, ServerProto};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use codec::TarantoolCodec;
use std::io;

pub struct TarantoolProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for TarantoolProto {
    type Request = Vec<u8>;
    type Response = Vec<u8>;

    type Transport = Framed<T, TarantoolCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TarantoolCodec {
            handshaked: false,
        }))
    }
}
