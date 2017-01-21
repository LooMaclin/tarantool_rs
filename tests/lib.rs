extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::tarantool::HeterogeneousElement;

#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::new("127.0.0.1:3301", "test", "test");
    tarantool_instance.auth();
    let test_tuple = [HeterogeneousElement::I8(1)];
    tarantool_instance.select("test", "primary", 10, 0, test_tuple.iter());
}