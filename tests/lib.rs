extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::tarantool::HeterogeneousElement;
use tarantool::iterator_type::IteratorType;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Group<'a> {
    id: u64,
    name: &'a str,
    year: u64,
}

#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::new("127.0.0.1:3301", "test", "test");
    tarantool_instance.auth();
    {
        let result = tarantool_instance.select(512, 0, 10, 0, IteratorType::Eq, (3)).unwrap();
        let group = Group {
            id: result.get(0).unwrap().as_u64().unwrap_or(0),
            name: result.get(1).unwrap().as_str().unwrap_or("fuck"),
            year: result.get(2).unwrap().as_u64().unwrap_or(0),
        };
        println!("Group: {:?}", group);
    }
}