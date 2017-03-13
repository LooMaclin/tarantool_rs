use request_type_key::RequestTypeKey;

pub trait Action {
    fn get(&self) -> (RequestTypeKey, Vec<u8>);
}