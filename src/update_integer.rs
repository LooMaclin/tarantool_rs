use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use integer_operation::IntegerOperation;
use FIX_STR_PREFIX;
use tarantool::Tarantool;
use byteorder::ByteOrder;
use action::Action;

#[derive(Debug)]
pub struct UpdateInteger<'a> {
    pub space: u16,
    pub index: u8,
    pub operation_type: IntegerOperation,
    pub field_number: u8,
    pub argument: u32,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for UpdateInteger<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Update, serialize(Value::Map(vec![
            (Value::from(Code::SpaceId as u8), Value::from(self.space)),
            (Value::from(Code::IndexId as u8), Value::from(self.index)),
            (Value::from(Code::Key as u8), Value::from(self.keys.clone())),
            (Value::from(Code::Tuple as u8), Value::from(vec![Value::from(vec![
                read_value(&mut &[
                    &[FIX_STR_PREFIX][..],
                    &[self.operation_type as u8][..],
                    &[self.field_number][..]].concat()[..]).unwrap(),
                Value::from(self.argument.clone())
            ]
            )]))
        ])))
    }
}
