use rmpv::Value;
use utils::serialize;
use request_type_key::RequestTypeKey;
use code::Code;
use action::Action;

#[derive(Debug)]
pub struct Auth {
    pub username: String,
    pub scramble: Vec<u8>,
}

impl Action for Auth {
    fn get(&self) -> (RequestTypeKey, Vec<u8>) {
        (RequestTypeKey::Auth,
         serialize(Value::Map(vec![(Value::from(Code::UserName as u8),
                                    Value::from(self.username.clone())),
                                   (Value::from(Code::Tuple as u8),
                                    Value::from(vec![Value::from("chap-sha1"),
                                                     Value::Binary(self.scramble.clone())]))])))
    }
}
