#[macro_use]
extern crate log;
extern crate tarantool;

use tarantool::{Value, SyncClient, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation,
                Upsert, UpsertOperation};

fn main() {
    let mut tarantool_instance = SyncClient::auth("127.0.0.1:3301", "test", "test")
        .unwrap_or_else(|err| {
            panic!("err: {}", err);
        });
}