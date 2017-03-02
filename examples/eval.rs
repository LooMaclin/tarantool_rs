extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let function_argument = vec![];
    let result = tarantool_instance.eval("test()", function_argument).unwrap_or_else(|err| {
        panic!("Tarantool eval error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}