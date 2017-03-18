use iterator_type::IteratorType;
use rmpv::{ValueRef, Value};
use utils::{header, request, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use sync_client::SyncClient;
use byteorder::ByteOrder;
use hex_slice::AsHex;
use action::Action;

#[derive(Debug)]
pub struct Insert {
    pub space: u64,
    pub keys: Vec<Value>,
}

impl Action for Insert {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Insert,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(self.keys.clone()))])))
    }
}
