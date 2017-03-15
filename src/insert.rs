use iterator_type::IteratorType;
use rmpv::{ValueRef, Value};
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use byteorder::ByteOrder;
use hex_slice::AsHex;
use action::Action;

#[derive(Debug)]
pub struct Insert<'a> {
    pub space: u64,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for Insert<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Insert,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(self.keys.clone()))])))
    }
}
