use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
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

#[derive(Debug)]
pub struct UpdateCommon<'a> {
    pub space: u16,
    pub index: u8,
    pub operation_type: CommonOperation,
    pub field_number: u8,
    pub argument: Value,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for UpdateCommon<'a> {
    fn get(&self)
                      -> (RequestTypeKey, Vec<u8>)
    {
        let keys_buffer = serialize_keys(self.keys.clone());
        let mut serialized_argument = serialize_keys(self.argument.clone());
        let mut body = [&[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[self.index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91, 0x93, FIX_STR_PREFIX, self.operation_type as u8, self.field_number][..],
            &serialized_argument[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        (RequestTypeKey::Update, body)
    }
}

