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
use rmpv::decode::value::read_value;
use std::clone::Clone;
use rmpv::ValueRef;
use rmpv::decode::value_ref::read_value_ref;
use response::Response;
use header::Header;
use select::Select;
use insert::Insert;
use upsert::Upsert;
use update_integer::UpdateInteger;
use update_string::UpdateString;
use update_common::UpdateCommon;
use delete::Delete;
use call::Call;
use replace::Replace;
use eval::Eval;
use action::Action;
use {TARANTOOL_SPACE_ID, TARANTOOL_INDEX_ID, TARANTOOL_SPACE_ID_KEY_NUMBER,
     TARANTOOL_INDEX_ID_KEY_NUMBER, CHAP_SHA_1};
use {Utf8String, Integer};
use utils::{header, serialize, process_response, scramble, build_auth_body, build_request, read_length, read_payload};
use state::State;

#[derive(Debug)]
pub struct SyncClient<'a> {
    pub state: State<'a>,
    pub descriptor: TcpStream,
}

impl<'a> SyncClient<'a> {
    pub fn auth<S>(address: S, user: S, password: S) -> Result<SyncClient<'a>, String>
        where S: Into<Cow<'a, str>>
    {
        let mut stream = TcpStream::connect("127.0.0.1:3301").unwrap();
        let mut buf = [0; 128];
        stream.read(&mut buf);
        let mut tarantool = SyncClient {
            state: State {
                address: address.into(),
                user: user.into(),
                password: password.into(),
                greeting_packet: GreetingPacket::new(String::from_utf8(buf[64..108].to_vec()).unwrap(),
                                                     String::from_utf8(buf[..64].to_vec()).unwrap()),
                request_id: 0,
            },
            descriptor: stream,
        };
        let scramble = scramble(&*tarantool.state.greeting_packet.salt, &*tarantool.state.password);
        let id = 0;
        let header = header(RequestTypeKey::Auth, id);
        let body = build_auth_body(tarantool.state.user.clone(), &scramble);
        let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        write_u32(&mut &mut encoded_request_length[..],
                  (header.len() + body.len()) as u32)
            .ok()
            .unwrap();
        let request = [&encoded_request_length[..], &header[..], &body[..]].concat();
        match SyncClient::get_response(&request, &mut tarantool.descriptor).body {
            Some(data) => Err(String::from_utf8(data).unwrap()),
            None => Ok(tarantool),
        }
    }



    pub fn request<I>(&mut self, request_body: &I) -> Result<Value, Utf8String>
        where I: Action
    {
        let request = build_request(request_body, self.state.get_id() as u32);
        let response = SyncClient::get_response(&request, &mut self.descriptor);
        process_response(&response)
    }

    pub fn get_response<I>(request: &[u8], mut descriptor: &mut I) -> Response
        where I: Write + Read
    {
        let write_result = descriptor.write(&request);
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

    pub fn fetch_space_id<I>(&mut self, space_name: I) -> Result<u64, String>
        where I: Into<Utf8String>
    {
        match self.request(&Select {
                                space: TARANTOOL_SPACE_ID,
                                index: TARANTOOL_SPACE_ID_KEY_NUMBER,
                                limit: 1,
                                offset: 0,
                                iterator: IteratorType::Eq,
                                keys: vec![Value::String(space_name.into())],
                            }) {
            Ok(data) => {
                println!("DATA: {:?}", data);
                match data[0][0].as_u64() {
                    Some(space_id) => Ok(space_id),
                    None => Err(String::from("Space not found")),
                }
            }
            Err(err) => Err(err.into_str().unwrap()),
        }
    }

    pub fn fetch_index_id<I, K>(&mut self, space_id: I, index_name: K) -> Result<u64, String>
        where I: Into<Integer>,
              K: Into<Utf8String>
    {
        match self.request(&Select {
                                space: TARANTOOL_INDEX_ID,
                                index: TARANTOOL_INDEX_ID_KEY_NUMBER,
                                limit: 1,
                                offset: 0,
                                iterator: IteratorType::Eq,
                                keys: vec![Value::Integer(space_id.into()),
                                           Value::String(index_name.into())],
                            }) {
            Ok(data) => {
                match data[0][1].as_u64() {
                    Some(index_id) => Ok(index_id),
                    None => Err(String::from("Space not found")),
                }
            }
            Err(err) => Err(err.into_str().unwrap()),
        }
    }
}
