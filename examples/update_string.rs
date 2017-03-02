extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::operation::StringOperation;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_string(512, 0, (3), 1, 2, 2, "FUCK").unwrap_or_else(|err| {
        panic!("Tarantool update string error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}