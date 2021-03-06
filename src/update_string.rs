use rmpv::Value;
use utils::serialize;
use request_type_key::RequestTypeKey;
use code::Code;
use FIX_STR_PREFIX;
use string_operation::StringOperation;
use std::borrow::Cow;
use action::Action;
use rmpv::decode::read_value;

#[derive(Debug)]
pub struct UpdateString<'a> {
    pub space: u64,
    pub index: u64,
    pub field_number: u64,
    pub position: u64,
    pub offset: u64,
    pub argument: Cow<'a, str>,
    pub keys: Vec<Value>,
}

impl<'a> Action for UpdateString<'a> {
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
                    &[StringOperation::Splice as u8][..],
                ].concat()[..]).unwrap(),
                    Value::from(self.field_number),
                    Value::from(self.position),
                    Value::from(self.offset),
                Value::from(self.argument.clone())
            ])]))])))
    }
}
