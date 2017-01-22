extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::tarantool::HeterogeneousElement;
use tarantool::iterator_type::IteratorType;
use std::borrow::Cow;
#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::new("127.0.0.1:3301", "test", "test");
    tarantool_instance.auth();
    {
        tarantool_instance.select(512, 0, 10, 0, IteratorType::Eq, (3));
    }
}