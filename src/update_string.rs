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
        let keys_buffer = serialize(self.keys.clone());
        let wrapped_argument = Value::String(self.argument.clone().into());
        let mut serialized_argument = serialize(wrapped_argument);
        let mut body = [&[0x84][..],
                        &[Code::SpaceId as u8][..],
                        &[0xCD, 0x0, 0x0][..],
                        &[Code::IndexId as u8][..],
                        &[self.index][..],
                        &[Code::Key as u8][..],
                        &keys_buffer[..],
                        &[Code::Tuple as u8][..],
                        &[0x91,
                          0x95,
                          FIX_STR_PREFIX,
                          StringOperation::Splice as u8,
                          self.field_number,
                          self.position,
                          self.offset][..],
                        &serialized_argument[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        (RequestTypeKey::Update, body)
    }
}
