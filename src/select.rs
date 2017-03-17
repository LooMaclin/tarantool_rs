use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use byteorder::ByteOrder;
use serde::Serialize;
use std::net::TcpStream;
use std::io::{Read, Write};
use tarantool::Tarantool;
use action::Action;

#[derive(Debug)]
pub struct Select {
    pub space: u64,
    pub index: u64,
    pub limit: u64,
    pub offset: u64,
    pub iterator: IteratorType,
    pub keys: Vec<Value>,
}

impl Action for Select {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Select,
         serialize(Value::Map(vec![
            (Value::from(Code::SpaceId as u8), Value::from(self.space)),
            (Value::from(Code::IndexId as u8), Value::from(self.index)),
            (Value::from(Code::Limit as u8), Value::from(self.limit)),
            (Value::from(Code::Offset as u8), Value::from(self.offset)),
            (Value::from(Code::Iterator as u8), Value::from(self.iterator as u8)),
            (Value::from(Code::Key as u8), Value::from(self.keys.clone())),
        ])))
    }
}
