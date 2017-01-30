use std::borrow::Cow;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;
use std::str::from_utf8;

use base64::{decode as decode_base64};
use sha1::{Sha1};
use rustc_serialize::{Encodable, Decodable};
use rmp_serialize::{Encoder, Decoder};
use rmp::encode::{write_u32, write_str};
use rmp::decode::{read_array_len};
use hex_slice::AsHex;
use byteorder::{ByteOrder, BigEndian};
use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use greeting_packet::GreetingPacket;
use code::Code;
use request_type_key::RequestTypeKey;
use iterator_type::IteratorType;
use rmpv::Value;
use rmpv::decode::value::{read_value, Error};
use std::clone::Clone;
use rmpv::ValueRef;
use rmpv::decode::value_ref::read_value_ref;


#[derive(Debug)]
pub struct Tarantool<'a> {
    address: Cow<'a, str>,
    user: Cow<'a, str>,
    password: Cow<'a, str>,
    greeting_packet: GreetingPacket<'a>,
    request_id: u32,
    socket: TcpStream,
}

#[derive(Debug, Clone)]
pub struct Response {
    header: Header,
    body: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Header {
    code: u32,
    sync: u64,
    schema_id:  u32,
}

impl<'a> Tarantool<'a> {
    pub fn auth<S>(address: S, user: S, password: S) -> Result<Tarantool<'a>, String>
        where S: Into<Cow<'a, str>> {
        let mut stream = TcpStream::connect("127.0.0.1:3301").unwrap();
        let mut buf = [0; 128];
        stream.read(&mut buf);
        let mut tarantool = Tarantool {
            address: address.into(),
            user: user.into(),
            password: password.into(),
            greeting_packet: GreetingPacket::new(
                String::from_utf8(buf[64..108].to_vec()).unwrap(),
                String::from_utf8(buf[..64].to_vec()).unwrap(),
            ),
            request_id: 0,
            socket: stream,
        };
        let scramble = Tarantool::scramble(&*tarantool.greeting_packet.salt, &*tarantool.password);
        println!("scramble (size: {}): {:#X}",&scramble.len(), &scramble.as_hex());
        let id = tarantool.get_id();
        let header = tarantool.header(RequestTypeKey::Auth, id);
        let mut chap_sha1_encoded = Vec::new();
        "chap-sha1".encode(&mut Encoder::new(&mut &mut chap_sha1_encoded[..]));
        let body = Tarantool::build_auth_body(tarantool.user.clone(), &scramble);
        match tarantool.request(&header, &body).body {
            Some(data) => {
                Err(String::from_utf8(data).unwrap())
            },
            None => {
                Ok(tarantool)
            }
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.request_id+=1;
        self.request_id
    }

    pub fn header(&self, command: RequestTypeKey, request_id: u32) -> Vec<u8> {
        let mut encoded_header = [
            &[0x82][..],
            &[Code::RequestType as u8][..],
            &[command as u8][..],
            &[Code::Sync as u8][..],
            &[0, 0, 0, 0, 0]].concat();
        write_u32(&mut &mut encoded_header[4..], request_id).ok().unwrap();
        encoded_header
    }

    pub fn request(&mut self, header: &[u8], body: &[u8]) -> Response {
        let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        write_u32(&mut &mut encoded_request_length[..],
                  (header.len() + body.len()) as u32).ok().unwrap();
        let request = [&encoded_request_length[..],&header[..],&body[..]].concat();
        self.socket.write(&request);
        let response_length = self.read_length();
        let payload = self.read_payload(response_length);
        println!("Greeting: {:?}", &self.greeting_packet);
        println!("request(size: {}): {:#X}", &request.len(), &request.as_hex());
        println!("length(size: {}): {:#X}", &encoded_request_length.len(), &encoded_request_length.as_hex());
        println!("header(size: {}): {:#X}", &header.len(), &header.as_hex());
        println!("body(size: {}): {:#X}", &body.len(), &body.as_hex());
        println!("payload(size: {}): {:#X}", &payload.len(), &payload.as_hex());
        println!("payload(as text): {}", String::from_utf8_lossy(&payload));
        let header = Header {
          code: BigEndian::read_u32(&payload[3..8]),
          sync: BigEndian::read_u64(&payload[9..17]),
          schema_id: BigEndian::read_u32(&payload[19..23]),
        };
        println!("body: {:#X}", &payload[..23].as_hex());
        Response {
            header: header,
            body:
            if payload.len() > 24 {
                Some(payload[23..payload.len()].to_vec())
            } else {
                Option::None
            },
        }
    }

    pub fn read_length(&mut self) -> u32 {
        let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        self.socket.read(&mut packet_length);
        let mut decoder = Decoder::new(&packet_length[..]);
        let mut length = Decodable::decode(&mut decoder).unwrap();
        length
    }

    pub fn read_payload(&mut self, length: u32) -> Vec<u8> {
        let mut payload = vec![0u8; length as usize];
        self.socket.read(&mut payload);
        payload
    }

    pub fn scramble<S>(salt: S, password: S) -> Vec<u8>
        where S: Into<Cow<'a, str>> {
        let decoded_salt = &decode_base64(&salt.into()).unwrap()[..];
        let mut step_1 = Sha1::new();
        step_1.update(&(password.into()[..]).as_bytes());
        let mut step_2 = Sha1::new();
        step_2.update(&step_1.digest().bytes());
        let mut step_3 = Sha1::new();
        step_3.update(&[&decoded_salt[..20], &step_2.digest().bytes()].concat());
        let digest_1 = step_1.digest().bytes();
        let digest_3 = step_3.digest().bytes();
        (0..20).into_iter()
            .map(|n| {
                digest_1[n] ^ digest_3[n]
            })
            .collect::<Vec<u8>>()
    }

    fn build_auth_body<S>(username: S, scramble: &[u8]) -> Vec<u8>
        where S: Into<Cow<'a,str>> {
        let mut encoded_username = Vec::new();
        write_str(&mut encoded_username, &username.into());
        [
            &[0x82][..],
            &[Code::UserName as u8][..],
            &encoded_username[..],
            &[Code::Tuple as u8, 0x92, 0xA9][..],
            &"chap-sha1".as_bytes(),
            &[0xC4, 0x14][..],
            &scramble[..]
        ].concat()
    }

    pub fn select<I>(&mut self, space: u16, index: u8, limit: u8, offset: u8, iterator: IteratorType, keys: I ) -> Result<Vec<Value>, String>
    where I: Serialize {
        let mut keys_buffer = Vec::new();
        keys.serialize(&mut Serializer::new(&mut keys_buffer));
        if keys_buffer.len() == 1 {
            keys_buffer = [
                &[0x91][..],
                &keys_buffer[..]
            ].concat();
        }
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Select, request_id);
        println!("KEYS BUFFER: {:#X}", &keys_buffer.as_hex());
        let mut body = [
            &[0x86][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Limit as u8][..],
            &[limit][..],
            &[Code::Offset as u8][..],
            &[offset][..],
            &[Code::Iterator as u8][..],
            &[iterator as u8][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        let data = response.body.ok_or("Some error...:(")?;
        let code = read_value(&mut &data[1..2]).unwrap().as_u64().unwrap();
        let data = read_value(&mut &data[2..]).unwrap();
        if code == 48 {
            Ok(if let Value::Array(result) = data { result } else { vec![] })
        } else {
            Err(if let Value::String(result) = data { result } else { "Unrecognized error".to_string() })
        }
    }
}

#[cfg(test)]
mod test {

    use super::Tarantool;
    use hex_slice::AsHex;

    #[test]
    fn scramble_result() {
        let scramble = Tarantool::scramble("WPE4wY2+RTBuFvElfHawAheh37sa58XKR/ZEOvgRsa8=", "123");
        assert_eq!([0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC,
                   0xDA, 0x0B, 0x86, 0xBD, 0x06, 0x2A, 0x69], &scramble[..]);
    }

    #[test]
    fn auth_body_result() {
        let auth_body = Tarantool::build_auth_body(
            "test",
            &[0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC,
            0xDA, 0x0B, 0x86, 0xBD, 0x06, 0x2A, 0x69][..]);
        assert_eq!(&[0x82, 0x23, 0xA4, 0x74, 0x65, 0x73, 0x74, 0x21, 0x92, 0xA9, 0x63, 0x68, 0x61,
            0x70, 0x2D, 0x73, 0x68, 0x61, 0x31, 0xC4, 0x14, 0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE,
            0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC, 0xDA, 0xB, 0x86, 0xBD, 0x6, 0x2A, 0x69][..],
        &auth_body[..]);
    }
}