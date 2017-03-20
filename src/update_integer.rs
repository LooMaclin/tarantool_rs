use iterator_type::IteratorType;
use rmpv::Value;
use utils::{header, serialize, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use integer_operation::IntegerOperation;
use FIX_STR_PREFIX;
use sync_client::SyncClient;
use byteorder::ByteOrder;
use action::Action;
use rmpv::decode::read_value;

#[derive(Debug)]
pub struct UpdateInteger {
    pub space: u64,
    pub index: u64,
    pub operation_type: IntegerOperation,
    pub field_number: u64,
    pub argument: u64,
    pub keys: Vec<Value>,
}

impl Action for UpdateInteger {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Update,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::IndexId as u8), Value::from(self.index)),
                                   (Value::from(Code::Key as u8),
                                    Value::from(self.keys.clone())),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(vec![Value::from(vec![
                read_value(&mut &[
                    &[FIX_STR_PREFIX][..],
                    &[self.operation_type as u8][..]].concat()[..]).unwrap(),
                    Value::from(self.field_number),
                    Value::from(self.argument)
            ])]))])))
    }
}
