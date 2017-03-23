use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use utils::read_length;
use hex_slice::AsHex;
use greeting_packet::GreetingPacket;
use rmpv::{Utf8String, Value};
use utils::{build_request, header, scramble, get_response, process_response};
use request_type_key::RequestTypeKey;
use rmp::encode::write_u32;
use insert::Insert;
use rmpv::decode::read_value;
use action::Action;
use std::marker::PhantomData;
use std::io::{Error, ErrorKind};
use async_response::AsyncResponse;
use action_type::ActionType;
use std::borrow::Cow;

#[derive(Debug)]
pub struct TarantoolCodec
{
    pub tarantool_handshake_received: bool,
}

impl Decoder for TarantoolCodec
{
    type Item = (RequestId, AsyncResponse);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("=== START DECODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        if self.tarantool_handshake_received {
            println!("HANDSHAKE RECEIVED SCOPE");
            if buf.len() < 5 {
                return Ok(None);
            } else {
                let length = read_length(&mut &buf.as_ref()[..5]);
                println!("Length: {}, size: {}", length as usize, length as usize + 5);
                if buf.len() == length as usize + 5 {
                    let mut incoming_object = buf.split_to(length as usize + 5);
                    println!("incoming object (size: {}): {:#X}", incoming_object.len(), incoming_object.as_hex());
                    let deserialized_incoming_object = get_response(&mut incoming_object.as_ref());
                    let request_id = deserialized_incoming_object.header.sync;
                    println!("Deserialized incoming object: {:?}", deserialized_incoming_object);
                    let deserialized_incoming_object = process_response(&deserialized_incoming_object);
                    println!("Deserialized incoming object: {:?}", deserialized_incoming_object);
                    return Ok(Some((request_id, AsyncResponse::Normal(deserialized_incoming_object))));
                }
            }
        } else {
            println!("HANDSHAKE NOT RECEIVED SCOPE");
            if buf.len() == 128 {
                let raw_greeting = buf.split_to(128);
                let salt = raw_greeting[64..108].to_vec();
                let scramble = scramble(String::from_utf8(salt).unwrap(), "test".to_string());
                self.tarantool_handshake_received = true;
                return Ok(Some((0, AsyncResponse::Handshake(scramble))));
            }
        }

        println!("Incoming buffer after (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        println!("=== END DECODE ===");
        Ok(None)
    }
}

impl Encoder for TarantoolCodec
{
    type Item = (RequestId, ActionType);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, ActionType), buf: &mut BytesMut) -> io::Result<()> {
        println!("=== START ENCODE ===");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        let (request_id, msg) = msg;
        let request = build_request(msg, request_id);
        buf.reserve(request.len());
        buf.put_slice(&request);
        println!("Incoming buffer after (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        println!("=== END ENCODE ===");
        Ok(())
    }
}
