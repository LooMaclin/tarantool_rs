use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use common_operation::CommonOperation;
use FIX_STR_PREFIX;
use std::borrow::Cow;
use byteorder::ByteOrder;
use tarantool::Tarantool;
use action::Action;
use rmpv::decode::read_value;

#[derive(Debug)]
pub struct UpdateCommon<'a> {
    pub space: u64,
    pub index: u64,
    pub operation_type: CommonOperation,
    pub field_number: u8,
    pub argument: &'a Value,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for UpdateCommon<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Update,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::IndexId as u8), Value::from(self.index)),
                                   (Value::from(Code::Key as u8),
                                    Value::from(self.keys.clone())),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(vec![Value::from(vec![
                                            read_value(
                                                &mut &[&[FIX_STR_PREFIX][..],
                                                &[self.operation_type as u8][..]]
                                                    .concat()[..]).unwrap(),
                        Value::from(self.field_number),
                        Value::from(self.argument.clone())
                ])]))])))
    }
}
