use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use utils::read_length;
use hex_slice::AsHex;
use greeting_packet::GreetingPacket;
use rmpv::{Utf8String, Value};
use utils::{build_request, header, build_auth_body, scramble};
use request_type_key::RequestTypeKey;
use rmp::encode::write_u32;
use insert::Insert;
use rmpv::decode::read_value;
use action::Action;
use std::marker::PhantomData;

pub struct TarantoolCodec<A> where A: Action {
    _phantom: PhantomData<A>,
    pub handshaked: bool,
}

impl <A> Decoder for TarantoolCodec<A> where A: Action {
    type Item = (RequestId, Result<Value, Utf8String>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("buf len: {:?}", buf.len());
        println!("buf: {:#X}", buf.as_ref().as_hex());
        match self.handshaked {
            true => {

                if buf.len() < 5 {
                    return Ok(None);
                }

                let length = read_length(&mut buf.as_ref());
                println!("length: {}", length);
                println!("MESSAGE SIZE: {}", length+5);
                if buf.len() == (length + 5) as usize {
                    println!("fuck");
                    return Ok(Some((1, Ok(Value::from("HAHAHA")))));
                }
            },
            false => {
                if buf.len() < 128 {
                    return Ok(None)
                } else if buf.len() == 128 {
                    self.handshaked = true;
                    let greeting = GreetingPacket::new(String::from_utf8(buf[64..108].to_vec()).unwrap(),
                                        String::from_utf8(buf[..64].to_vec()).unwrap());
                    println!("greeting: {:?}", greeting);
                    let scramble = scramble(greeting.salt, "test".into());
                    let id = 0;
                    let header = header(RequestTypeKey::Auth, id);
                    let body = build_auth_body("test", &scramble);
                    let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
                    write_u32(&mut &mut encoded_request_length[..],
                              (header.len() + body.len()) as u32)
                        .ok()
                        .unwrap();
                    let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
                    return Ok(Some((0, Ok(read_value(&mut &request[..]).unwrap()))))
                }
            }
        }
        Ok(None)
    }
}

impl<A> Encoder for TarantoolCodec<A> where A: Action {
    type Item = (RequestId, A);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, A), buf: &mut BytesMut) -> io::Result<()> {
//        let len = 4 + buf.len() + 1;
//        buf.reserve(len);
//
          let (request_id, msg) = msg;
//
//        buf.put_u32::<BigEndian>(request_id as u32);
//        buf.put_slice(&msg[..]);
//        buf.put_u8(b'\n');
        let body = msg.get();
        buf.reserve(body.1.len());
        buf.put_slice(&body.1[..]);
        Ok(())
    }
}
