use rmpv::{Value, Utf8String};

#[derive(Debug)]
pub enum AsyncResponse {
    Handshake(Vec<u8>),
    Normal(Result<Value, Utf8String>),
}
