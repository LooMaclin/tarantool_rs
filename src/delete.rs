use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use byteorder::ByteOrder;
use action::Action;

#[derive(Debug)]
pub struct Delete<'a> {
    pub space: u16,
    pub index: u8,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for Delete<'a> {
    fn get(&self)
                      -> (RequestTypeKey, Vec<u8>)
    {
        let wrapped_keys = Value::Array(self.keys.clone());
        let keys_buffer = serialize_keys(wrapped_keys);
        let mut body = [&[0x83][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[self.index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        (RequestTypeKey::Delete, body)
    }
}

