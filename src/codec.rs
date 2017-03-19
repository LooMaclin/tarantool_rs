use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::RequestId;
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use utils::read_length;
use hex_slice::AsHex;
use greeting_packet::GreetingPacket;


pub struct TarantoolCodec {
    pub handshaked: bool,
}

impl Decoder for TarantoolCodec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<(RequestId, Vec<u8>)>, io::Error> {
        println!("buf len: {:?}", buf.len());
        println!("buf: {:#X}", buf.as_ref().as_hex());
        match self.handshaked {
            true => {

                if buf.len() < 5 {
                    return Ok(None);
                }

                let length = read_length(&mut buf.as_ref());
                println!("length: {:?}", length);

                if buf.len() == (length + 5) as usize {
                    println!("fuck");
                    return Ok(Some((1, vec![])));
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
                    return Ok(Some((1, buf[..].to_vec())))
                }
            }
        }
        Ok(None)
    }
}

impl Encoder for TarantoolCodec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn encode(&mut self, msg: (RequestId, Vec<u8>), buf: &mut BytesMut) -> io::Result<()> {
        let len = 4 + buf.len() + 1;
        buf.reserve(len);

        let (request_id, msg) = msg;

        buf.put_u32::<BigEndian>(request_id as u32);
        buf.put_slice(&msg[..]);
        buf.put_u8(b'\n');

        Ok(())
    }
}
