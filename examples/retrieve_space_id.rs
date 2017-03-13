extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });

    let tuples = tarantool_instance.fetch_space_id("tester".into());
    println!("Tester space id: {}", tuples);
}

