use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;

#[derive(Debug, Builder)]
pub struct Delete<'a> {
    space: u16,
    index: u8,
    keys: &'a Vec<Value>,
}

impl<'a> Delete<'a> {
    pub fn perform<I>(&self, state: &mut Tarantool)
                      -> Result<Value, String>
        where I: Serialize
    {
        let wrapped_keys = Value::Array(self.keys);
        let keys_buffer = serialize_keys(wrapped_keys);
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Delete, request_id);
        let mut body = [&[0x83][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[self.index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

