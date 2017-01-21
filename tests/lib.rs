extern crate tarantool;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::tarantool::HeterogeneousElement;
use std::borrow::Cow;
#[test]
fn tarantool_with_builder() {
    let mut tarantool_instance = Tarantool::new("127.0.0.1:3301", "test", "test");
    tarantool_instance.auth();
    let test_array = [
        HeterogeneousElement::I8(-1),
        HeterogeneousElement::I16(-2),
        HeterogeneousElement::I32(-3),
        HeterogeneousElement::I64(-4),
        HeterogeneousElement::STRING(Cow::Borrowed("Borrowed string")),
        HeterogeneousElement::STRING(Cow::Owned("Owned string".to_string())),
        HeterogeneousElement::U8(0),
        HeterogeneousElement::U16(1),
        HeterogeneousElement::U32(2),
        HeterogeneousElement::U64(3),
        HeterogeneousElement::BOOLEAN(true)
    ];
    let test_tuple = (HeterogeneousElement::I8(-1),
                      HeterogeneousElement::I16(-2),
                      HeterogeneousElement::I32(-3),
                      HeterogeneousElement::I64(-4),
                      HeterogeneousElement::STRING(Cow::Borrowed("Borrowed string")),
                      HeterogeneousElement::STRING(Cow::Owned("Owned string".to_string())),
                      HeterogeneousElement::U8(0),
                      HeterogeneousElement::U16(1),
                      HeterogeneousElement::U32(2),
                      HeterogeneousElement::U64(3),
                      HeterogeneousElement::BOOLEAN(true));
    let test_vector = vec![HeterogeneousElement::I8(-1),
                           HeterogeneousElement::I16(-2),
                           HeterogeneousElement::I32(-3),
                           HeterogeneousElement::I64(-4),
                           HeterogeneousElement::STRING(Cow::Borrowed("Borrowed string")),
                           HeterogeneousElement::STRING(Cow::Owned("Owned string".to_string())),
                           HeterogeneousElement::U8(0),
                           HeterogeneousElement::U16(1),
                           HeterogeneousElement::U32(2),
                           HeterogeneousElement::U64(3),
                           HeterogeneousElement::BOOLEAN(true)];
    tarantool_instance.select("test", "primary", 10, 0, &test_array);
    tarantool_instance.select("test", "primary", 10, 0, &test_tuple);
    tarantool_instance.select("test", "primary", 10, 0, &test_vector);
}