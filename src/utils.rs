use response::Response;
use rmpv::{Value, Utf8String};
use rmpv::decode::read_value;
use std::borrow::Cow;
use code::Code;
use {TARANTOOL_SPACE_ID, TARANTOOL_INDEX_ID, TARANTOOL_SPACE_ID_KEY_NUMBER,
     TARANTOOL_INDEX_ID_KEY_NUMBER, CHAP_SHA_1};
use std::io::{Read, Write};
use request_type_key::RequestTypeKey;
use rmp::encode::{write_u32, write_str};
use header::Header;
use serde::Serialize;
use hex_slice::AsHex;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use rmp_serde::{Serializer, Deserializer};
use rmp_serialize::{Encoder, Decoder};
use rustc_serialize::{Encodable, Decodable};
use sha1::Sha1;
use base64::decode as decode_base64;


pub fn process_response(response: &Response) -> Result<Value, Utf8String> {
    let data = response.body
        .as_ref()
        .ok_or("Body is empty.")?;
    match read_value(&mut &data[..]).unwrap() {
        Value::Map(mut data) => {
            let (code, content) = data.remove(0);
            let code = match code {
                Value::Integer(code) => code,
                _ => panic!("Operation result code is't number."),
            };
            if code.as_u64().unwrap() == 48 {
                Ok(content)
            } else {
                match content {
                    Value::String(result) => Err(result),
                    _ => Err(Utf8String::from("Error content is't string.")),
                }
            }
        }
        _ => Err(Utf8String::from("Read data error.")),
    }
}

pub fn read_payload<I>(length: u32, descriptor: &mut I) -> Vec<u8>
    where I: Read
{
    let mut payload = vec![0u8; length as usize];
    debug!("PAYLOAD BEFORE: {:?}", payload);
    descriptor.read(&mut payload);
    debug!("PAYLOAD AFTER: {:?}", payload);
    payload
}

pub fn header(command: RequestTypeKey, request_id: u32) -> Vec<u8> {
    serialize(Value::Map(vec![(Value::from(Code::RequestType as u8), Value::from(command as u8)),
                              (Value::from(Code::Sync as u8), Value::from(request_id))]))
}

pub fn request<I>(header: &[u8], body: &[u8], mut descriptor: &mut I) -> Response
    where I: Write + Read
{
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

pub fn serialize<I>(keys: I) -> Vec<u8>
    where I: Serialize
{
    let mut keys_buffer = Vec::new();
    keys.serialize(&mut Serializer::new(&mut keys_buffer));
    keys_buffer
}

pub fn read_length<I>(stream: &mut I) -> u32
    where I: Read
{
    let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
    stream.read(&mut packet_length);
    let mut decoder = Decoder::new(&packet_length[..]);
    let mut length = Decodable::decode(&mut decoder).unwrap();
    length
}

pub fn scramble<'a, S>(salt: S, password: S) -> Vec<u8>
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
    (0..20).into_iter().map(|n| digest_1[n] ^ digest_3[n]).collect::<Vec<u8>>()
}

pub fn build_auth_body<'a, S>(username: S, scramble: &[u8]) -> Vec<u8>
    where S: Into<Cow<'a, str>>
{
    let mut encoded_username = Vec::new();
    write_str(&mut encoded_username, &username.into());
    [&[0x82][..],
     &[Code::UserName as u8][..],
     &encoded_username[..],
     &[Code::Tuple as u8, 0x92][..],
     &CHAP_SHA_1[..],
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
        let auth_body = build_auth_body("test",
                                        &[0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92,
                                          0x79, 0xCE, 0xC3, 0xFC, 0xDA, 0x0B, 0x86, 0xBD, 0x06,
                                          0x2A, 0x69]
                                             [..]);
        assert_eq!(&[0x82, 0x23, 0xA4, 0x74, 0x65, 0x73, 0x74, 0x21, 0x92, 0xA9, 0x63, 0x68,
                     0x61, 0x70, 0x2D, 0x73, 0x68, 0x61, 0x31, 0xC4, 0x14, 0xAC, 0x3F, 0xAD,
                     0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC, 0xDA, 0xB, 0x86,
                     0xBD, 0x6, 0x2A, 0x69]
                        [..],
                   &auth_body[..]);
    }

    #[test]
    fn read_length_test() {
        assert_eq!(5, read_length(&mut &[0xCE, 0x00, 0x00, 0x00, 0x5][..]));
    }
}
