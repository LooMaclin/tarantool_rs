use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use upsert_operation::UpsertOperation;
use tarantool::Tarantool;
use FIX_STR_PREFIX;
use byteorder::ByteOrder;
use action::Action;
use rmpv::decode::read_value;
use {Utf8String, Integer};

#[derive(Debug)]
pub struct Upsert<'a> {
    pub space: u64,
    pub keys: &'a Vec<Value>,
    pub operation_type: UpsertOperation,
    pub field_number: u64,
    pub argument: u64,
}

impl<'a> Action for Upsert<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        let body = Value::Map(
          vec![
              (Value::Integer((Code::SpaceId as u64).into()), Value::Integer(self.space.into())),
              (Value::Integer((Code::Tuple as u64).into()), Value::Array(self.keys.clone())),
              (Value::Integer((Code::OPS as u64).into()), Value::Array(
                      vec![
                          read_value(&mut &[&[FIX_STR_PREFIX][..], &[self.operation_type as u8][..]].concat()[..]).unwrap(),
                          Value::from(self.field_number),
                          Value::from(self.argument)
                  ]))
          ]);
        let body = serialize(body);
//        let keys_buffer = serialize(self.keys);
//        let wrapped_argument = Value::from(self.argument);
//        let mut serialized_argument = serialize(wrapped_argument);
//        let mut body =
//            [&[0x84][..],
//             &[Code::SpaceId as u8][..],
//             &[0xCD, 0x0, 0x0][..],
//             &[Code::Tuple as u8][..],
//             &keys_buffer[..],
//             &[Code::OPS as u8][..],
//             &[0x91, 0x93, FIX_STR_PREFIX, self.operation_type as u8, self.field_number][..],
//             &serialized_argument[..]]
//                .concat();
//        BigEndian::write_u16(&mut body[3..5], self.space);
        (RequestTypeKey::Upsert, body)
    }
}
