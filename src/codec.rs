use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use utils::read_length;
use hex_slice::AsHex;
use greeting_packet::GreetingPacket;
use rmpv::{Utf8String, Value};
use utils::{build_request, header, scramble, get_response};
use request_type_key::RequestTypeKey;
use rmp::encode::write_u32;
use insert::Insert;
use rmpv::decode::read_value;
use action::Action;
use std::marker::PhantomData;
use std::io::{Error, ErrorKind};
use async_response::AsyncResponse;

pub struct TarantoolCodec<A>
    where A: Action
{
    pub _phantom: PhantomData<A>,
    pub tarantool_handshake_received: bool,
}

impl<A> Decoder for TarantoolCodec<A>
    where A: Action
{
    type Item = (RequestId, AsyncResponse);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("=== START DECODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        if self.tarantool_handshake_received {
            if buf.len() < 5 {
                return Ok(None);
            } else {
                let length = read_length(&mut buf.as_ref());
                println!("Object length: {}", length);
                if buf.len() == (length + 5) as usize {
                    let incoming_object = buf.split_to(length as usize + 5);
                    return Ok(Some((1, AsyncResponse::Normal(Ok(Value::from("HAHAHA"))))));
                }
            }
        } else {
            if buf.len() == 128 {
                let raw_greeting = buf.split_to(128);
                let salt = raw_greeting[64..108].to_vec();
                self.tarantool_handshake_received = true;
                return Ok(Some((0, AsyncResponse::Handshake(salt))));
            }
        }

        println!("Incoming buffer after (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        println!("=== END DECODE ===");
        Ok(None)
    }
}

impl<A> Encoder for TarantoolCodec<A>
    where A: Action
{
    type Item = (RequestId, A);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, A), buf: &mut BytesMut) -> io::Result<()> {
        println!("=== START ENCODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        let (request_id, msg) = msg;
        let request = build_request(&msg, request_id);
        buf.reserve(request.len());
        buf.put_slice(&request);
        println!("Incoming buffer after (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        println!("=== END ENCODE ===");
        Ok(())
    }
}
