use rmpv::Value;
use rmpv::Utf8String;
use tarantool::{Tarantool};

pub trait ToMsgPack {
    fn get_msgpack_representation(&self) -> Vec<Value>;
}


pub trait Space {
    fn insert<I>(data: I, connection: &mut Tarantool) -> Result<Value, Utf8String>;
}