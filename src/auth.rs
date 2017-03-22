use iterator_type::IteratorType;
use rmpv::{ValueRef, Value};
use utils::{header, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use sync_client::SyncClient;
use byteorder::ByteOrder;
use hex_slice::AsHex;
use action::Action;
use rmpv::decode::read_value;
use CHAP_SHA_1;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Auth<'a> {
    pub username: Cow<'a,str>,
    pub scramble: Vec<u8>,
}

impl <'a> Action for Auth<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Auth, serialize(Value::Map(vec![
            (Value::from(Code::UserName as u8), Value::from(self.username)),
            (Value::from(Code::Tuple as u8), Value::from(vec![
                read_value(&mut &[&CHAP_SHA_1[..], &[0xC4, 0x14][..], &self.scramble[..]].concat()[..]).unwrap()
            ]))
        ])))
    }
}
