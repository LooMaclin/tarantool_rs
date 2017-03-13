extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete};

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
    println!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
    println!("Replace result: {:?}", Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));
    println!("Delete result: {:?}", Delete {
        space: 512,
        index: 0,
        keys: &vec![Value::from(3)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));
    println!("Insert result: {:?}",
    Insert {
        space: 512,
        keys: &vec![Value::from(9)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));
}