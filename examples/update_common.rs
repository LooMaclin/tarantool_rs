extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::operation::CommonOperation;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_common(512, 0, (3), CommonOperation::Assign, 2, Value::from(2015)).unwrap_or_else(|err| {
        panic!("Tarantool update common error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}