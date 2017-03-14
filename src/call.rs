use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;
use action::Action;
use {Integer, Utf8String};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Call<'a> {
    pub function_name: Cow<'a,str>,
    pub keys: &'a Vec<Value>,
}

impl<'a> Action for Call<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
//        let wrapped_keys = Value::Array(self.keys.clone());
//        let keys_buffer = serialize(wrapped_keys);
//        let function_name = serialize(Value::String(self.function_name.into()));
//        let mut body = [&[0x82][..],
//                        &[Code::FunctionName as u8][..],
//                        &function_name[..],
//                        &[Code::Tuple as u8][..],
//                        &keys_buffer[..]]
//            .concat();
        (RequestTypeKey::Call, serialize(
            Value::Map(vec![
                (Value::from(Code::FunctionName as u8), Value::from(self.function_name.clone())),
                (Value::from(Code::Tuple as u8), Value::from(self.keys.clone()))
            ])
        ))
    }
}
