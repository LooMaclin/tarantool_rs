extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,CommonOperation, Call, Eval, UpdateString};

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });
    let error_handler = |err| panic!("Tarantool error: {}", err);
    let tuples = tarantool_instance.request(Select {
        space: 512,
        index: 0,
        limit: 100,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    })
        .unwrap_or_else(&error_handler);
    println!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
    println!("Replace result: {:?}", tarantool_instance.request(Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    })
        .unwrap_or_else(&error_handler));
    println!("Delete result: {:?}", tarantool_instance.request(Delete {
        space: 512,
        index: 0,
        keys: &vec![Value::from(3)]
    })
        .unwrap_or_else(&error_handler));
    println!("Common-Update result: {:?}", tarantool_instance.request(UpdateCommon {
        space: 512,
        index: 0,
        operation_type: CommonOperation::Assign,
        field_number: 3,
        argument: Value::String(String::from("Test Update Common Assign")),
        keys: &vec![Value::from(6)]
    })
        .unwrap_or_else(&error_handler));
    println!("Call result: {:?}", tarantool_instance.request(Call {
        function_name: "test",
        keys: &vec![]
    }).unwrap_or_else(&error_handler));
    println!("Eval result: {:?}", tarantool_instance.request(Eval {
        expression: r#"return 5+5"#,
        keys: &vec![]
    }).unwrap_or_else(&error_handler));
    println!("String-Update result: {:?}", tarantool_instance.request(UpdateString {
        space: 512,
        index: 0,
        field_number: 1,
        position: 3,
        offset: 3,
        argument: "TEST UPDATE STRING".into(),
        keys: &vec![Value::from(2)]
    }).unwrap_or_else(&error_handler));
    println!("Insert result: {:?}",
    tarantool_instance.request(Insert {
        space: 512,
        keys: &vec![Value::from(9)]
    })
        .unwrap_or_else(&error_handler));
}