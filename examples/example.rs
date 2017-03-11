extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::iterator_type::IteratorType;
use tarantool::tarantool::{select};
use tarantool::Value;

fn main() {
    let error_handler = |err| panic!("Tarantool error: {}", err);
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });
    let tuples = select()
            .space(512 as u16)
            .index(0)
            .limit(10)
            .offset(0)
            .iterator(IteratorType::All)
            .keys(&vec![Value::from(3)])
            .build()
            .unwrap()
            .perform(&mut tarantool_instance)
            .unwrap_or_else(&error_handler);
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
}