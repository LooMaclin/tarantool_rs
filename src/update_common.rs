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

#[derive(Debug, Builder)]
pub struct UpdateCommon<'a> {
    space: u16,
    index: u8,
    operation_type: CommonOperation,
    field_number: u8,
    argument: Value,
    keys: &'a Vec<Value>,
}

impl<'a> UpdateCommon<'a> {
    pub fn perform<I>(&self, state: &mut Tarantool)
                      -> Result<Value, String>
        where I: Serialize
    {
        let keys_buffer = serialize_keys(self.keys);
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Update, request_id);
        let mut serialized_argument = serialize_keys(self.argument);
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
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

