use rmpv::Value;
use rmpv::Utf8String;
use sync_client::SyncClient;

pub trait ToMsgPack {
    fn get_msgpack_representation(&self) -> Vec<Value>;
}


pub trait Space {
    fn insert<I>(data: I, connection: &mut SyncClient) -> Result<Value, Utf8String>;
}
