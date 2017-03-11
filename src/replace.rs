use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;

#[derive(Debug, Builder)]
pub struct Replace<'a> {
    space: u16,
    keys: &'a Vec<Value>,
}

impl<'a> Replace<'a> {
    pub fn perform<I>(&self, state: &mut Tarantool)
                      -> Result<Value, String>
        where I: Serialize
    {
        let mut keys_buffer = Vec::new();
        let wrapped_keys = serialize_keys(Value::Array(self.keys));
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Replace, request_id);
        let mut body = [&[0x82][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

