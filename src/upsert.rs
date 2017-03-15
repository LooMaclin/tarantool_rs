use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use upsert_operation::UpsertOperation;
use tarantool::Tarantool;
use FIX_STR_PREFIX;
use byteorder::ByteOrder;
use action::Action;
use rmpv::decode::read_value;
use {Utf8String, Integer};

#[derive(Debug)]
pub struct Upsert<'a> {
    pub space: u64,
    pub keys: &'a Vec<Value>,
    pub operation_type: UpsertOperation,
    pub field_number: u64,
    pub argument: u64,
}

impl<'a> Action for Upsert<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Upsert,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(self.keys.clone())),
                                   (Value::from(Code::OPS as u8),
                                    Value::from(vec![Value::from(vec![
                        read_value(&mut &[
                            &[FIX_STR_PREFIX][..],
                            &[self.operation_type as u8][..]].concat()[..]).unwrap(),
                        Value::from(self.field_number),
                        Value::from(self.argument)
                                   ])]))])))
    }
}
