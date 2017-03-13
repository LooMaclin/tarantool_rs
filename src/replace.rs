use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use byteorder::ByteOrder;

#[derive(Debug)]
pub struct Replace<'a> {
    pub space: u16,
    pub keys: &'a Vec<Value>,
}

impl<'a> Replace<'a> {
    pub fn perform(&self, state: &mut Tarantool)
                      -> Result<Value, String>
    {
        let keys_buffer = serialize_keys(Value::Array(self.keys.clone()));
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

