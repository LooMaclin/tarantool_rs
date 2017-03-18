extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });

    let error_handler = |err| panic!("Tarantool error: {}", err);

    let insert = Insert {
        space: 512,
        keys: vec![Value::Array(vec![Value::from(9)]), Value::Array(vec![Value::from(10)])]
    };

    println!("Insert result: {:?}", tarantool_instance.request(&insert).unwrap_or_else(&error_handler));
}