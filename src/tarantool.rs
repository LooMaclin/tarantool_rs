use std::borrow::Cow;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::Cursor;
use std::collections::HashMap;
use std::str::from_utf8;

use base64::decode as decode_base64;
use sha1::Sha1;
use rustc_serialize::{Encodable, Decodable};
use rmp_serialize::{Encoder, Decoder};
use rmp::encode::{write_u32, write_str};
use rmp::decode::read_array_len;
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
use response::Response;
use header::Header;
use select::{Select, SelectBuilder};
use insert::{Insert, InsertBuilder};
use upsert::{Upsert, UpsertBuilder};
use update_integer::{UpdateInteger, UpdateIntegerBuilder};
use update_string::{UpdateString, UpdateStringBuilder};
use update_common::{UpdateCommon, UpdateCommonBuilder};
use delete::{Delete,DeleteBuilder};
use call_16::{Call16,Call16Builder};
use call::{Call, CallBuilder};
use replace::{Replace, ReplaceBuilder};
use eval::{Eval, EvalBuilder};

#[derive(Debug)]
pub struct Tarantool<'a> {
    address: Cow<'a, str>,
    user: Cow<'a, str>,
    password: Cow<'a, str>,
    greeting_packet: GreetingPacket<'a>,
    request_id: u32,
    pub descriptor: TcpStream,
}

impl<'a> Tarantool<'a> {
    pub fn auth<S>(address: S, user: S, password: S) -> Result<Tarantool<'a>, String>
        where S: Into<Cow<'a, str>>
    {
        let mut stream = TcpStream::connect("127.0.0.1:3301").unwrap();
        let mut buf = [0; 128];
        stream.read(&mut buf);
        let mut tarantool = Tarantool {
            address: address.into(),
            user: user.into(),
            password: password.into(),
            greeting_packet: GreetingPacket::new(String::from_utf8(buf[64..108].to_vec()).unwrap(),
                                                 String::from_utf8(buf[..64].to_vec()).unwrap()),
            request_id: 0,
            descriptor: stream,
        };
        debug!("Tarantool: {:?}", tarantool);
        let scramble = scramble(&*tarantool.greeting_packet.salt, &*tarantool.password);
        let id = tarantool.get_id();
        let header = header(RequestTypeKey::Auth, id);
        let mut chap_sha1_encoded = Vec::new();
        "chap-sha1".encode(&mut Encoder::new(&mut &mut chap_sha1_encoded[..]));
        let body = build_auth_body(tarantool.user.clone(), &scramble);
        match request(&header, &body, &mut tarantool.descriptor).body {
            Some(data) => Err(String::from_utf8(data).unwrap()),
            None => Ok(tarantool),
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.request_id += 1;
        self.request_id
    }
}

pub fn select<'a>() -> SelectBuilder<'a> {
    SelectBuilder::default()
}

pub fn insert<'a>() -> InsertBuilder<'a> {
    InsertBuilder::default()
}

pub fn upsert<'a>() -> UpsertBuilder<'a> {
    UpsertBuilder::default()
}

pub fn update_integer<'a>() -> UpdateIntegerBuilder<'a> {
    UpdateIntegerBuilder::default()
}

pub fn update_common<'a>() -> UpdateCommonBuilder<'a> {
    UpdateCommonBuilder::default()
}

pub fn update_string<'a>() -> UpdateStringBuilder<'a> {
    UpdateStringBuilder::default()
}

pub fn eval<'a>() -> EvalBuilder<'a> {
    EvalBuilder::default()
}

pub fn replace<'a>() -> ReplaceBuilder<'a> {
    ReplaceBuilder::default()
}

pub fn call<'a>() -> CallBuilder<'a> {
    CallBuilder::default()
}

pub fn call_16<'a>() -> Call16Builder<'a> {
    Call16Builder::default()
}

pub fn delete<'a>() -> DeleteBuilder<'a> {
    DeleteBuilder::default()
}


pub fn process_response(response: &Response) -> Result<Value, String> {
    let data = response.body.as_ref().ok_or("Body is empty.")?;
    match read_value(&mut &data[..]).unwrap() {
        Value::Map(mut data) => {
            let (code, content) = data.remove(0);
            let code = match code {
                Value::U64(code) => code,
                _ => panic!("Operation result code is't number."),
            };
            if code == 48 {
                Ok(content)
            } else {
                match content {
                    Value::String(result) => Err(result),
                    _ => Err("Error content is't string.".to_string()),
                }
            }
        }
        _ => Err("Read data error.".to_string()),
    }
}

pub fn read_payload<I>(length: u32, descriptor: &mut I) -> Vec<u8>
    where I: Read {
    let mut payload = vec![0u8; length as usize];
    debug!("PAYLOAD BEFORE: {:?}", payload);
    descriptor.read(&mut payload);
    debug!("PAYLOAD AFTER: {:?}", payload);
    payload
}

pub fn header(command: RequestTypeKey, request_id: u32) -> Vec<u8> {
    let mut encoded_header = [&[0x82][..],
        &[Code::RequestType as u8][..],
        &[command as u8][..],
        &[Code::Sync as u8][..],
        &[0, 0, 0, 0, 0]]
        .concat();
    write_u32(&mut &mut encoded_header[4..], request_id).ok().unwrap();
    encoded_header
}

pub fn request<I>(header: &[u8], body: &[u8], mut descriptor: &mut I) -> Response
    where I: Write + Read {
    let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
    write_u32(&mut &mut encoded_request_length[..],
              (header.len() + body.len()) as u32)
        .ok()
        .unwrap();
    let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
    let write_result = descriptor.write(&request);
    debug!("WRITE RESULT: {:?}", write_result);
    let response_length = read_length(&mut descriptor);
    debug!("RESPONSE LENGTH: {:?}", response_length);
    let payload = read_payload(response_length, &mut descriptor);
    debug!("PAYLOAD: {:?}", payload);
    debug!("request(size: {}): {:#X}",
    &request.len(),
    &request.as_hex());
    debug!("length(size: {}): {:#X}",
    &encoded_request_length.len(),
    &encoded_request_length.as_hex());
    debug!("header(size: {}): {:#X}", &header.len(), &header.as_hex());
    debug!("body(size: {}): {:#X}", &body.len(), &body.as_hex());
    debug!("payload(size: {}): {:#X}",
    &payload.len(),
    &payload.as_hex());
    debug!("payload(as text): {}", String::from_utf8_lossy(&payload));
    let header = Header {
        code: BigEndian::read_u32(&payload[3..8]),
        sync: BigEndian::read_u64(&payload[9..17]),
        schema_id: BigEndian::read_u32(&payload[19..23]),
    };
    Response {
        header: header,
        body: if payload.len() > 24 {
            Some(payload[23..payload.len()].to_vec())
        } else {
            Option::None
        },
    }
}

pub fn serialize_keys<I>(keys: I) -> Vec<u8>
    where I: Serialize
{
    let mut keys_buffer = Vec::new();
    keys.serialize(&mut Serializer::new(&mut keys_buffer));
    keys_buffer
}

fn read_length<I>(stream: &mut I) -> u32
    where I: Read
{
    let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
    stream.read(&mut packet_length);
    let mut decoder = Decoder::new(&packet_length[..]);
    let mut length = Decodable::decode(&mut decoder).unwrap();
    length
}

fn scramble<'a, S>(salt: S, password: S) -> Vec<u8>
    where S: Into<Cow<'a, str>>
{
    let decoded_salt = &decode_base64(&salt.into()).unwrap()[..];
    let mut step_1 = Sha1::new();
    step_1.update(&(password.into()[..]).as_bytes());
    let mut step_2 = Sha1::new();
    step_2.update(&step_1.digest().bytes());
    let mut step_3 = Sha1::new();
    step_3.update(&[&decoded_salt[..20], &step_2.digest().bytes()].concat());
    let digest_1 = step_1.digest().bytes();
    let digest_3 = step_3.digest().bytes();
    (0..20)
        .into_iter()
        .map(|n| digest_1[n] ^ digest_3[n])
        .collect::<Vec<u8>>()
}

fn build_auth_body<'a, S>(username: S, scramble: &[u8]) -> Vec<u8>
    where S: Into<Cow<'a, str>>
{
    let mut encoded_username = Vec::new();
    write_str(&mut encoded_username, &username.into());
    [&[0x82][..],
        &[Code::UserName as u8][..],
        &encoded_username[..],
        &[Code::Tuple as u8, 0x92, 0xA9][..],
        &"chap-sha1".as_bytes(),
        &[0xC4, 0x14][..],
        &scramble[..]]
        .concat()
}

#[cfg(test)]
mod test {

    use super::{scramble, build_auth_body, read_length};
    use hex_slice::AsHex;

    #[test]
    fn scramble_test() {
        let scramble = scramble("WPE4wY2+RTBuFvElfHawAheh37sa58XKR/ZEOvgRsa8=", "123");
        assert_eq!([0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC,
                    0xDA, 0x0B, 0x86, 0xBD, 0x06, 0x2A, 0x69],
                   &scramble[..]);
    }

    #[test]
    fn build_auth_body_test() {
        let auth_body =
            build_auth_body("test",
                                       &[0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92,
                                         0x79, 0xCE, 0xC3, 0xFC, 0xDA, 0x0B, 0x86, 0xBD, 0x06,
                                         0x2A, 0x69][..]);
        assert_eq!(&[0x82, 0x23, 0xA4, 0x74, 0x65, 0x73, 0x74, 0x21, 0x92, 0xA9, 0x63, 0x68,
                     0x61, 0x70, 0x2D, 0x73, 0x68, 0x61, 0x31, 0xC4, 0x14, 0xAC, 0x3F, 0xAD,
                     0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC, 0xDA, 0xB,
                     0x86, 0xBD, 0x6, 0x2A, 0x69][..],
                   &auth_body[..]);
    }

    #[test]
    fn read_length_test() {
        assert_eq!(5,
                   read_length(&mut &[0xCE, 0x00, 0x00, 0x00, 0x5][..]));
    }
}
