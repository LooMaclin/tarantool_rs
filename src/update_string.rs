use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use FIX_STR_PREFIX;
use string_operation::StringOperation;
use std::borrow::Cow;
use byteorder::ByteOrder;

#[derive(Debug, Builder)]
pub struct UpdateString<'a> {
    space: u16,
    index: u8,
    field_number: u8,
    position: u8,
    offset: u8,
    argument: Cow<'a,str>,
    keys: &'a Vec<Value>,
}

impl<'a> UpdateString<'a> {
    pub fn perform<I>(&self, state: &mut Tarantool)
                      -> Result<Value, String>
        where I: Serialize
    {
        let keys_buffer = serialize_keys(self.keys.clone());
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Update, request_id);
        let wrapped_argument = Value::String(self.argument.clone().into());
        let mut serialized_argument = serialize_keys(wrapped_argument);
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
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

