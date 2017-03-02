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
use operation::IntegerOperation;
use operation::StringOperation;
use operation::CommonOperation;
use operation::UpsertOperation;
use operation::FIX_STR_PREFIX;

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
        let response_length = Tarantool::read_length(&mut self.socket);
        let payload = &self.read_payload(response_length)[..response_length as usize];
        debug!("Greeting: {:?}", &self.greeting_packet);
        debug!("request(size: {}): {:#X}", &request.len(), &request.as_hex());
        debug!("length(size: {}): {:#X}", &encoded_request_length.len(), &encoded_request_length.as_hex());
        debug!("header(size: {}): {:#X}", &header.len(), &header.as_hex());
        debug!("body(size: {}): {:#X}", &body.len(), &body.as_hex());
        debug!("payload(size: {}): {:#X}", &payload.len(), &payload.as_hex());
        debug!("payload(as text): {}", String::from_utf8_lossy(&payload));
        let header = Header {
          code: BigEndian::read_u32(&payload[3..8]),
          sync: BigEndian::read_u64(&payload[9..17]),
          schema_id: BigEndian::read_u32(&payload[19..23]),
        };
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

    fn read_length<I>(stream: &mut I) -> u32 where I: Read {
        let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        stream.read(&mut packet_length);
        let mut decoder = Decoder::new(&packet_length[..]);
        let mut length = Decodable::decode(&mut decoder).unwrap();
        length
    }

    pub fn read_payload(&mut self, length: u32) -> [u8; 8192] {
        let mut payload = [0u8; 8192];
        self.socket.read(&mut payload);
        payload
    }

    fn scramble<S>(salt: S, password: S) -> Vec<u8>
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

    pub fn select<I>(&mut self, space: u16, index: u8, limit: u8, offset: u8, iterator: IteratorType, keys: I ) -> Result<Value, String>
    where I: Serialize {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Select, request_id);
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
        Tarantool::process_response(&response)
    }

    pub fn insert(&mut self, space: u16, keys: Vec<Value>) -> Result<Value, String> {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = Tarantool::serialize_keys(wrapped_keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Insert, request_id);
        let mut body = [
            &[0x82][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn replace(&mut self, space: u16, keys: Vec<Value>) -> Result<Value, String> {
        let mut keys_buffer = Vec::new();
        let wrapped_keys = Value::Array(keys);
        wrapped_keys.serialize(&mut Serializer::new(&mut keys_buffer)).unwrap();
        if keys_buffer.len() == 1 {
            keys_buffer = [
                &[0x91][..],
                &keys_buffer[..]
            ].concat();
        }
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Replace, request_id);
        let mut body = [
            &[0x82][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn serialize_keys<I>(keys: I) -> Vec<u8> where I: Serialize {
        let mut keys_buffer = Vec::new();
        keys.serialize(&mut Serializer::new(&mut keys_buffer));
        if keys_buffer.len() == 1 {
            keys_buffer = [
                &[0x91][..],
                &keys_buffer[..]
            ].concat();
        }
        keys_buffer
    }

    pub fn process_response(response: &Response) -> Result<Value, String> {
        let data = response.body.as_ref().ok_or("Body is empty.")?;
        match read_value(&mut &data[..]).unwrap() {
            Value::Map(mut data) => {
                let (code, content) = data.remove(0);
                let code = match code {
                    Value::U64(code) => code,
                    _ => panic!("Operation result code is't number.")
                };
                if code == 48 {
                    Ok(content)
                } else {
                    match content {
                        Value::String(result) => Err(result),
                        _ => Err("Error content is't string.".to_string())
                    }
                }
            },
            _ => Err("Read data error.".to_string()),
        }
    }

    pub fn update_integer<I>(&mut self, space: u16, index: u8, keys: I, operation_type: IntegerOperation, field_number: u8, argument: u32) -> Result<Value, String>
        where I: Serialize {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Update, request_id);
        let wrapped_argument = Value::from(argument);
        let mut serialized_argument = Vec::new();
        wrapped_argument.serialize(&mut Serializer::new(&mut serialized_argument)).unwrap();
        let mut body = [
            &[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91, 0x93, FIX_STR_PREFIX, operation_type as u8, field_number][..],
            &serialized_argument[..],
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn update_string<I, S>(&mut self, space: u16, index: u8, keys: I, field_number: u8, position: u8, offset: u8, argument: S) -> Result<Value, String>
        where I: Serialize,
              S: Into<Cow<'a,str>> + Serialize {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Update, request_id);
        let wrapped_argument = Value::String(argument.into().into_owned());
        let mut serialized_argument = Vec::new();
        wrapped_argument.serialize(&mut Serializer::new(&mut serialized_argument)).unwrap();
        let mut body = [
            &[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91, 0x95, FIX_STR_PREFIX, StringOperation::Splice as u8, field_number, position, offset][..],
            &serialized_argument[..],
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn update_common<I>(&mut self, space: u16, index: u8, keys: I, operation_type: CommonOperation, field_number: u8, argument: Value) -> Result<Value, String>
        where I: Serialize {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Update, request_id);
        let wrapped_argument = argument;
        let mut serialized_argument = Vec::new();
        wrapped_argument.serialize(&mut Serializer::new(&mut serialized_argument)).unwrap();
        let mut body = [
            &[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91, 0x93, FIX_STR_PREFIX, operation_type as u8, field_number][..],
            &serialized_argument[..],
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn delete(&mut self, space: u16, index: u8, keys: Vec<Value>) -> Result<Value, String> {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = Tarantool::serialize_keys(wrapped_keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Delete, request_id);
        let mut body = [
            &[0x83][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn call_16(&mut self, function_name: &'static str, keys: Vec<Value>) -> Result<Value, String> {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = Tarantool::serialize_keys(wrapped_keys);
        let function_name = Tarantool::serialize_keys(Value::String(function_name.into()));
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Call16, request_id);
        let mut body = [
            &[0x82][..],
            &[Code::FunctionName as u8][..],
            &function_name[..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]
        ].concat();
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn call(&mut self, function_name: &'static str, keys: Vec<Value>) -> Result<Value, String> {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = Tarantool::serialize_keys(wrapped_keys);
        let function_name = Tarantool::serialize_keys(Value::String(function_name.into()));
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Call, request_id);
        let mut body = [
            &[0x82][..],
            &[Code::FunctionName as u8][..],
            &function_name[..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]
        ].concat();
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn eval(&mut self, expression: &'static str, keys: Vec<Value>) -> Result<Value, String> {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = Tarantool::serialize_keys(wrapped_keys);
        let function_name = Tarantool::serialize_keys(Value::String(expression.into()));
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Eval, request_id);
        let mut body = [
            &[0x82][..],
            &[Code::EXPR as u8][..],
            &function_name[..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]
        ].concat();
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
    }

    pub fn upsert<I>(&mut self, space: u16, keys: I, operation_type: UpsertOperation, field_number: u8, argument: u32) -> Result<Value, String>
        where I: Serialize {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let request_id = self.get_id();
        let header = self.header(RequestTypeKey::Update, request_id);
        let wrapped_argument = Value::from(argument);
        let mut serialized_argument = Vec::new();
        wrapped_argument.serialize(&mut Serializer::new(&mut serialized_argument)).unwrap();
        let mut body = [
            &[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91, 0x93, FIX_STR_PREFIX, operation_type as u8, field_number][..],
            &serialized_argument[..],
        ].concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = self.request(&header, &body);
        Tarantool::process_response(&response)
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

    #[test]
    fn read_length() {
        assert_eq!(5, Tarantool::read_length(&mut &[0xCE, 0x00, 0x00, 0x00, 0x5][..]));
    }
}