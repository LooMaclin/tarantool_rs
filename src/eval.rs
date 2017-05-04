use rmpv::Value;
use utils::serialize;
use request_type_key::RequestTypeKey;
use code::Code;
use action::Action;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Eval<'a> {
    pub expression: Cow<'a, str>,
    pub keys: Vec<Value>,
}

impl<'a> Action for Eval<'a> {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Eval,
         serialize(Value::Map(vec![(Value::from(Code::EXPR as u8),
                                    Value::from(self.expression.clone())),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(self.keys.clone()))])))
    }
}
