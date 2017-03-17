use rmpv::Value;
use rmpv::Utf8String;
use tarantool::{Tarantool};

pub trait Space {
    fn get_msgpack_representation(&self) -> Vec<Value>;
    fn insert(data: Vec<Value>, connection: &mut Tarantool) -> Result<Value, Utf8String>;
}