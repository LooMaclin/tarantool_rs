use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use FIX_STR_PREFIX;
use string_operation::StringOperation;
use std::borrow::Cow;
use byteorder::ByteOrder;
use action::Action;

#[derive(Debug)]
pub struct UpdateString<'a> {
    pub space: u16,
    pub index: u8,
    pub field_number: u8,
    pub position: u8,
    pub offset: u8,
    pub argument: Cow<'a, str>,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for UpdateString<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Update, serialize(Value::Map(vec![
            (Value::from(Code::SpaceId as u8), Value::from(self.space)),
            (Value::from(Code::IndexId as u8), Value::from(self.index)),
            (Value::from(Code::Key as u8), Value::from(self.keys.clone())),
            Value::from(Code::Tuple as u8), Value::from(vec![Value::from(vec![
                read_value(&mut &[
                    &[FIX_STR_PREFIX][..],
                    &[StringOperation::Splice as u8][..],
                    &[self.field_number][..],
                    &[self.position as u8][..],
                    &[self.offset as u8][..]].concat()[..]).unwrap(),
                Value::from(self.argument.clone())
            ]
            )])
        ])))
    }
}
