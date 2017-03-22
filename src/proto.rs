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
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TarantoolCodec {
            _phantom: PhantomData,
            handshaked: false,
        }))
    }
}
