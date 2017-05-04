use response::Response;
use rmpv::{Value, Utf8String};
use rmpv::decode::read_value;
use code::Code;
use std::io::Read;
use request_type_key::RequestTypeKey;
use rmp::encode::write_u32;
use header::Header;
use serde::Serialize;
use hex_slice::AsHex;
use byteorder::BigEndian;
use byteorder::ByteOrder;
use rmp_serde::Serializer;
use rmp_serialize::Decoder;
use rustc_serialize::Decodable;
use sha1::Sha1;
use base64::decode as decode_base64;
use action::Action;
use action_type::ActionType;

pub fn build_request(request_body: ActionType, request_id: u64) -> Vec<u8> {
    match request_body {
        ActionType::Auth(auth) => {
            let (request_type, body) = auth.get();
            let header = header(request_type, request_id);
            debug!("Request header: {:#X}", header.as_hex());
            debug!("Request body: {:#X}", body.as_hex());
            let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
            write_u32(&mut &mut encoded_request_length[..],
                      (header.len() + body.len()) as u32)
                    .ok()
                    .unwrap();
            let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
            request
        }
        ActionType::Select(select) => {
            let (request_type, body) = select.get();
            let header = header(request_type, request_id);
            debug!("Request header: {:#X}", header.as_hex());
            debug!("Request body: {:#X}", body.as_hex());
            let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
            write_u32(&mut &mut encoded_request_length[..],
                      (header.len() + body.len()) as u32)
                    .ok()
                    .unwrap();
            let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
            request
        }
        ActionType::Insert(insert) => {
            let (request_type, body) = insert.get();
            let header = header(request_type, request_id);
            debug!("Request header: {:#X}", header.as_hex());
            debug!("Request body: {:#X}", body.as_hex());
            let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
            write_u32(&mut &mut encoded_request_length[..],
                      (header.len() + body.len()) as u32)
                    .ok()
                    .unwrap();
            let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
            request
        }
    }
}

pub fn build_request_sync<I>(request_body: &I, request_id: u64) -> Vec<u8>
    where I: Action
{
    let (request_type, body) = request_body.get();
    let header = header(request_type, request_id);
    debug!("Request header: {:#X}", header.as_hex());
    debug!("Request body: {:#X}", body.as_hex());
    let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
    write_u32(&mut &mut encoded_request_length[..],
              (header.len() + body.len()) as u32)
            .ok()
            .unwrap();
    let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
    request
}

pub fn get_response<I>(mut descriptor: &mut I) -> Response
    where I: Read
{

    let response_length = read_length(&mut descriptor);
    let payload = read_payload(response_length, &mut descriptor);
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


pub fn process_response(response: &Response) -> Result<Value, Utf8String> {
    let data = response.body.as_ref().ok_or("Body is empty.")?;
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
    let descriptor_read_result = descriptor.read(&mut payload);
    descriptor_read_result.unwrap();
    debug!("PAYLOAD AFTER: {:?}", payload);
    payload
}

pub fn header(command: RequestTypeKey, request_id: u64) -> Vec<u8> {
    serialize(Value::Map(vec![(Value::from(Code::RequestType as u8), Value::from(command as u8)),
                              (Value::from(Code::Sync as u8), Value::from(request_id))]))
}



pub fn serialize<I>(keys: I) -> Vec<u8>
    where I: Serialize
{
    let mut keys_buffer = Vec::new();
    let keys_serializing_result = keys.serialize(&mut Serializer::new(&mut keys_buffer));
    keys_serializing_result.unwrap();
    keys_buffer
}

pub fn read_length<I>(stream: &mut I) -> u32
    where I: Read
{
    let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
    let read_result = stream.read(&mut packet_length);
    let mut decoder = Decoder::new(&packet_length[..read_result.unwrap()]);
    Decodable::decode(&mut decoder).unwrap()
}

pub fn scramble(salt: String, password: String) -> Vec<u8> {
    let decoded_salt = &decode_base64(&salt).unwrap()[..];
    let mut step_1 = Sha1::new();
    step_1.update(&(password[..]).as_bytes());
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
