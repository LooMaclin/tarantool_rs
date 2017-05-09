use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut};
use utils::read_length;
use hex_slice::AsHex;
use utils::{build_request, scramble, get_response, process_response};
use async_response::AsyncResponse;
use action_type::ActionType;

#[derive(Debug)]
pub struct TarantoolCodec {
    pub tarantool_handshake_received: bool,
    pub tarantool_auth_message_received: bool,
}

impl Decoder for TarantoolCodec {
    type Item = (RequestId, AsyncResponse);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("===DECODE======================================================================");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        if self.tarantool_handshake_received {
            println!("Tarantool handshake received scope...");
            if self.tarantool_auth_message_received {
                println!("Tarantool auth message recevied scope...");
                if buf.len() <= 5 {
                    return Ok(None);
                } else {
                    let length = read_length(&mut &buf.as_ref()[..5]);
                    println!("Length: {}, size: {}", length as usize, length as usize + 5);
                    if buf.len() >= length as usize + 5 {
                        let incoming_object = buf.split_to(length as usize + 5);
                        println!("incoming object (size: {}): {:#X}",
                                 incoming_object.len(),
                                 incoming_object.as_hex());
                        let raw_response_with_header = get_response(&mut incoming_object.as_ref());
                        let request_id = raw_response_with_header.header.sync;
                        println!("NORMAL REQUEST ID: {}", request_id);
                        println!("Deserialized raw response object: {:?}",
                                 raw_response_with_header);
                        let deserialized_incoming_object = process_response(&raw_response_with_header);
                        println!("Deserialized incoming object: {:?}",
                                 deserialized_incoming_object);
                        println!("Incoming buffer after (size: {}): {:#X} \n",
                                 buf.len(),
                                 buf.as_ref().as_hex());
                        return Ok(Some((request_id,
                                        AsyncResponse::Normal(deserialized_incoming_object))));
                    }
                }
            } else {
                println!("Tarantool auth message NOT recevied scope...");
                if buf.len() <= 5 {
                    return Ok(None);
                } else {
                    let length = read_length(&mut &buf.as_ref()[..5]);
                    println!("Length: {}, size: {}", length as usize, length as usize + 5);
                    if buf.len() >= length as usize + 5 {
                        let incoming_object = buf.split_to(length as usize + 5);
                        println!("incoming object (size: {}): {:#X}",
                                 incoming_object.len(),
                                 incoming_object.as_hex());
                        let raw_response_with_header = get_response(&mut incoming_object.as_ref());
                        let request_id = raw_response_with_header.header.sync;
                        println!("NORMAL REQUEST ID: {}", request_id);
                        println!("Deserialized raw response object: {:?}",
                                 raw_response_with_header);
                        let deserialized_incoming_object = process_response(&raw_response_with_header);
                        println!("Deserialized incoming object: {:?}",
                                 deserialized_incoming_object);
                        println!("Incoming buffer after (size: {}): {:#X} \n",
                                 buf.len(),
                                 buf.as_ref().as_hex());
                        match deserialized_incoming_object {
                            Ok(_) => {
                                self.tarantool_auth_message_received = true;
                                return Ok(None)
                            },
                            Err(_) => {
                                self.tarantool_auth_message_received = true;
                                return Ok(None)
                            }
                         }
                    }
                }
            }
        } else {
            println!("HANDSHAKE NOT RECEIVED SCOPE");
            if buf.len() == 128 {
                let raw_greeting = buf.split_to(128);
                let salt = raw_greeting[64..108].to_vec();
                let scramble = scramble(String::from_utf8(salt).unwrap(), "test".to_string());
                self.tarantool_handshake_received = true;
                println!("Incoming buffer after (size: {}): {:#X} \n",
                         buf.len(),
                         buf.as_ref().as_hex());
                return Ok(Some((0, AsyncResponse::Handshake(scramble))));
            }
        }

        println!("Incoming buffer after (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        Ok(None)
    }
}

impl Encoder for TarantoolCodec {
    type Item = (RequestId, ActionType);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, ActionType), buf: &mut BytesMut) -> io::Result<()> {
        println!("===ENCODE======================================================================");
        println!("Incoming buffer before (size: {}): {:#X} \n",
                 buf.len(),
                 buf.as_ref().as_hex());
        let (mut request_id, msg) = msg;
        match msg {
            ActionType::Auth(_) => {
                request_id = request_id + 1;
                let request = build_request(msg, request_id);
                buf.reserve(request.len());
                buf.put_slice(&request);
                println!("Incoming buffer after (size: {}): {:#X} \n",
                         buf.len(),
                         buf.as_ref().as_hex());
            }
            _ => {
                if self.tarantool_handshake_received && self.tarantool_auth_message_received {
                    request_id = request_id + 1;
                    let request = build_request(msg, request_id);
                    buf.reserve(request.len());
                    buf.put_slice(&request);
                    println!("Incoming buffer after (size: {}): {:#X} \n",
                             buf.len(),
                             buf.as_ref().as_hex());
                }
            }
        }

        Ok(())
    }
}

impl Drop for TarantoolCodec {
    fn drop(&mut self) {
        panic!("what a ****");
    }
}
