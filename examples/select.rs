extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::iterator_type::IteratorType;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.select(512, 0, 10, 0, IteratorType::All, (3)).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
}