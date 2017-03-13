extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });
    let tuples = Select {
        space: 512,
        index: 0,
        limit: 10,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(|err| panic!("Tarantool error: {}", err));
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
    Insert {
        space: 512,
        keys: &vec![Value::from(2433335)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(|err| panic!("Tarantool error: {}", err));
}