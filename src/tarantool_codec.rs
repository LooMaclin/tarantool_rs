use tokio_io::codec::{Decoder, Encoder};
use tokio_proto::multiplex::{RequestId};
use std::io;
use bytes::{BytesMut, BufMut, BigEndian};
use tarantool::read_length;

pub struct TarantoolCodec;

impl Decoder for TarantoolCodec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<(RequestId, Vec<u8>)>, io::Error> {
        // At least 5 bytes are required for a frame: 4 byte head + one byte
        // '\n'
        if buf.len() < 5 {
            return Ok(None);
        }

        let length = read_length(&mut buf.as_ref());
        println!("length: {:?}", length);
        println!("buf len: {:?}", buf.len());

        if buf.len() == (length+5) as usize {
            println!("fuck");
            return Ok(Some((1, vec![])))
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