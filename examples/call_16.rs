extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let function_argument = vec![Value::from(12)];
    let result = tarantool_instance.call_16("test", function_argument).unwrap_or_else(|err| {
        panic!("Tarantool call 16 error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}