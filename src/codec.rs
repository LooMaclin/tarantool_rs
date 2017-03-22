use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use utils::read_length;
use hex_slice::AsHex;
use greeting_packet::GreetingPacket;
use rmpv::{Utf8String, Value};
use utils::{build_request, header, build_auth_body, scramble, get_response};
use request_type_key::RequestTypeKey;
use rmp::encode::write_u32;
use insert::Insert;
use rmpv::decode::read_value;
use action::Action;
use std::marker::PhantomData;
use std::io::{Error, ErrorKind};

pub struct TarantoolCodec<A> where A: Action {
    pub _phantom: PhantomData<A>
}

impl <A> Decoder for TarantoolCodec<A> where A: Action {
    type Item = (RequestId, Result<Value, Utf8String>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("=== START DECODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n", buf.len(), buf.as_ref().as_hex());
                if buf.len() < 5 {
                    return Ok(None);
                } else {
                    let length = read_length(&mut buf.as_ref());
                    println!("LENGTH: {}", length);
                    if buf.len() == (length + 5) as usize {
                        buf.split_to(length as usize +5);
                        return Ok(Some((1, Ok(Value::from("HAHAHA")))));
                    }
                }
        println!("Incoming buffer after (size: {}): {:#X} \n", buf.len(), buf.as_ref().as_hex());
        println!("=== END DECODE ===");
        Ok(None)
    }
}

impl<A> Encoder for TarantoolCodec<A> where A: Action {
    type Item = (RequestId, A);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, A), buf: &mut BytesMut) -> io::Result<()> {
        println!("=== START ENCODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n", buf.len(), buf.as_ref().as_hex());
                let (request_id, msg) = msg;
                let request = build_request(&msg, request_id);
                buf.reserve(request.len());
                buf.put_slice(&request);
        println!("Incoming buffer after (size: {}): {:#X} \n", buf.len(), buf.as_ref().as_hex());
        println!("=== END ENCODE ===");
        Ok(())
    }
}
