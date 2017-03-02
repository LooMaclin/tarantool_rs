extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let delete_key = vec![Value::from(12)];
    let result = tarantool_instance.delete(512, 0, delete_key).unwrap_or_else(|err| {
        panic!("Tarantool delete error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}