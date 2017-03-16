use rmpv::Value;

pub trait Insertable {
    fn get_msgpack_representation(&self) -> Vec<Value>;
}