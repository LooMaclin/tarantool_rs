use rmpv::Value;
use utils::serialize;
use request_type_key::RequestTypeKey;
use code::Code;
use action::Action;

#[derive(Debug)]
pub struct Delete {
    pub space: u64,
    pub index: u64,
    pub keys: Vec<Value>,
}

impl Action for Delete {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Delete,
         serialize(Value::Map(vec![(Value::from(Code::SpaceId as u8), Value::from(self.space)),
                                   (Value::from(Code::IndexId as u8), Value::from(self.index)),
                                   (Value::from(Code::Key as u8),
                                    Value::from(self.keys.clone()))])))
    }
}
