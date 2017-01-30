extern crate tarantool;
extern crate rmpv;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::iterator_type::IteratorType;
use std::borrow::Cow;
use rmpv::Value;

#[derive(Debug)]
pub struct Group<'a> {
    id: u64,
    name: &'a str,
    year: u64,
}

fn print_group(result: &Vec<Value>) {
    let group = Group {
        id: result.get(0).unwrap_or(&Value::U64(0)).as_u64().unwrap(),
        name: result.get(1).unwrap().as_str().unwrap(),
        year: result.get(2).unwrap_or(&Value::U64(0)).as_u64().unwrap(),
    };
    println!("Group: {:?}", group);
}

#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.select(512, 0, 10, 0, IteratorType::Eq, (3)).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    for (index, tuple) in tuples.iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        print_group(tuple);
    }
}