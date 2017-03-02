extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::operation::IntegerOperation;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_integer(512, 0, (3), IntegerOperation::Addition, 2, 5).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}