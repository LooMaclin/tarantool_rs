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

#[derive(Debug)]
pub struct SyncClient<'a> {
    address: Cow<'a, str>,
    user: Cow<'a, str>,
    password: Cow<'a, str>,
    greeting_packet: GreetingPacket<'a>,
    request_id: u32,
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
            address: address.into(),
            user: user.into(),
            password: password.into(),
            greeting_packet: GreetingPacket::new(String::from_utf8(buf[64..108].to_vec()).unwrap(),
                                                 String::from_utf8(buf[..64].to_vec()).unwrap()),
            request_id: 0,
            descriptor: stream,
        };
        debug!("SyncClient: {:?}", tarantool);
        let scramble = scramble(&*tarantool.greeting_packet.salt, &*tarantool.password);
        let id = tarantool.get_id();
        let header = header(RequestTypeKey::Auth, id);
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

    pub fn request<I>(&mut self, request_body: &I) -> Result<Value, Utf8String>
        where I: Action
    {
        let (request_type, body) = request_body.get();
        let header = header(request_type, self.get_id());
        debug!("Request header: {:#X}", header.as_hex());
        debug!("Request body: {:#X}", body.as_hex());
        let response = request(&header, &body, &mut self.descriptor);
        process_response(&response)
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
                    Some(space_id) => {
                        Ok(space_id)
                    },
                    None => {
                        Err(String::from("Space not found"))
                    }
                }
            },
            Err(err) => {
                Err(err.into_str().unwrap())
            }
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
                    keys: vec![Value::Integer(space_id.into()), Value::String(index_name.into())],
                }) {
            Ok(data) => {
                match data[0][1].as_u64() {
                    Some(index_id) => {
                        Ok(index_id)
                    },
                    None => {
                        Err(String::from("Space not found"))
                    }
                }
            },
            Err(err) => {
                Err(err.into_str().unwrap())
            }
        }
    }
}


