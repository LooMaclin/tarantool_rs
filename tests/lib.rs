extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;

#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::new("127.0.0.1:3301", "loomaclin", "123");
    tarantool_instance.auth();
}