extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let inserting_value = vec![Value::from(6), Value::String("Black Some Sign".to_string()), Value::from(2005)];
    let result = tarantool_instance.replace(512, inserting_value).unwrap_or_else(|err| {
        panic!("Tarantool insert error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}