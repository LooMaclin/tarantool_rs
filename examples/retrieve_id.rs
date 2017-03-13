extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });

    let space_id = tarantool_instance.fetch_space_id("tester");
    println!("Tester space id: {}", space_id);

    let index_id = tarantool_instance.fetch_index_id(space_id, "primary");
    println!("Tester primary index id: {}", index_id);
}

