use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use byteorder::ByteOrder;
use action::Action;

#[derive(Debug)]
pub struct Replace<'a> {
    pub space: u64,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for Replace<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        let body = Value::Map(
            vec![
                (Value::Integer((Code::SpaceId as u64).into()), Value::Integer(self.space.into())),
                (Value::Integer((Code::Tuple as u64).into()), Value::from(self.keys.clone()))
            ]);
        let body = serialize(body);
//        let keys_buffer = serialize(Value::Array(self.keys.clone()));
//        let mut body = [&[0x82][..],
//                        &[Code::SpaceId as u8][..],
//                        &[0xCD, 0x0, 0x0][..],
//                        &[Code::Tuple as u8][..],
//                        &keys_buffer[..]]
//            .concat();
//        BigEndian::write_u16(&mut body[3..5], self.space);
        (RequestTypeKey::Replace, body)
    }
}
