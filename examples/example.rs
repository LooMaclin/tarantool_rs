extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });
    let error_handler = |err| panic!("Tarantool error: {}", err);
    let tuples = Select {
        space: 512,
        index: 0,
        limit: 10,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler);
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
    Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler);
    Insert {
        space: 512,
        keys: &vec![Value::from(2433335)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler);
}