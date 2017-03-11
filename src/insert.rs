use iterator_type::IteratorType;
use rmpv::{ValueRef, Value};
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use byteorder::ByteOrder;
use hex_slice::AsHex;

#[derive(Debug, Builder)]
pub struct Insert<'a> {
    space: u16,
    keys: &'a Vec<Value>,
}

impl<'a> Insert<'a> {
    pub fn perform(&self, state: &mut Tarantool)
                      -> Result<Value, String>
    {
        let wrapped_keys = Value::Array(self.keys.clone());
        let keys_buffer = serialize_keys(wrapped_keys);
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Insert, request_id);
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

