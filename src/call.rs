use iterator_type::IteratorType;
use rmpv::Value;
use utils::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use sync_client::SyncClient;
use action::Action;
use {Integer, Utf8String};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Call<'a> {
    pub function_name: Cow<'a, str>,
    pub keys: Vec<Value>,
}

impl<'a> Action for Call<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Call,
         serialize(Value::Map(vec![(Value::from(Code::FunctionName as u8),
                                    Value::from(self.function_name.clone())),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(self.keys.clone()))])))
    }
}
