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
    pub _phantom: PhantomData<A>,
    pub handshaked: bool,
    pub auth_request_sended: bool,
    pub auth_request: Option<Vec<u8>>,
}

impl <A> Decoder for TarantoolCodec<A> where A: Action {
    type Item = (RequestId, Result<Value, Utf8String>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("Incoming buffer (size: {}): {:#X}", buf.len(), buf.as_ref().as_hex());
        match self.handshaked {
            true => {
                println!("FUCK THE SYSTYEM");
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
                } else {
                    let greeting = GreetingPacket::new(String::from_utf8(buf[64..108].to_vec()).unwrap(),
                                        String::from_utf8(buf[..64].to_vec()).unwrap());
                    println!("Greeting: {:?}", greeting);
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
                    self.auth_request = Some(request)
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
        println!("=== ENCODE ===");
        match self.handshaked {
            true => {
                println!("HANDSHAKED : TRUE");
                let (request_id, msg) = msg;
                let request = build_request(&msg, request_id);
                buf.reserve(request.len());
                buf.put_slice(&request);
                Ok(())
            },
            false => {
                println!("HANDSHAKED : FALSE");
                if let Some(ref request_data) = self.auth_request {
                    match get_response(&request_data, &mut &buf[..]).body {
                        Some(data) => return Err(Error::new(ErrorKind::PermissionDenied, String::from_utf8(data).unwrap())),
                        None => {
                            self.handshaked = true;
                            return Ok(())
                        },
                    }
                }
                Ok(())
            }
        }

    }
}
