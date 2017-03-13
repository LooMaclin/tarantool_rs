extern crate tarantool;

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

fn main() {

    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });

    let error_handler = |err| panic!("Tarantool error: {}", err);

    let select = Select {
        space: 512,
        index: 0,
        limit: 100,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    };

    let tuples = tarantool_instance.request(&select).unwrap_or_else(&error_handler);

    println!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }

    let replace = Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    };

    println!("Replace result: {:?}", tarantool_instance.request(&replace).unwrap_or_else(&error_handler));

    let delete = Delete {
        space: 512,
        index: 0,
        keys: &vec![Value::from(3)]
    };

    println!("Delete result: {:?}", tarantool_instance.request(&delete).unwrap_or_else(&error_handler));

    let update_common = UpdateCommon {
        space: 512,
        index: 0,
        operation_type: CommonOperation::Assign,
        field_number: 3,
        argument: Value::String(String::from("Test Update Common Assign")),
        keys: &vec![Value::from(6)]
    };

    println!("Common-Update result: {:?}", tarantool_instance.request(&update_common).unwrap_or_else(&error_handler));

    let call = Call {
        function_name: "test",
        keys: &vec![]
    };

    println!("Call result: {:?}", tarantool_instance.request(&call).unwrap_or_else(&error_handler));

    let eval = Eval {
        expression: r#"return 5+5"#,
        keys: &vec![]
    };

    println!("Eval result: {:?}", tarantool_instance.request(&eval).unwrap_or_else(&error_handler));

    let update_string = UpdateString {
        space: 512,
        index: 0,
        field_number: 1,
        position: 3,
        offset: 3,
        argument: "TEST UPDATE STRING".into(),
        keys: &vec![Value::from(2)]
    };

    println!("String-Update result: {:?}", tarantool_instance.request(&update_string).unwrap_or_else(&error_handler));

    let update_integer = UpdateInteger {
        space: 512,
        index: 0,
        operation_type: IntegerOperation::Addition,
        field_number: 2,
        argument: 1,
        keys: &vec![Value::from(4)]
    };

    println!("Integer-Update result: {:?}", tarantool_instance.request(&update_integer).unwrap_or_else(&error_handler));

    let upsert = Upsert {
        space: 512,
        keys: &vec![Value::from(5)],
        operation_type: UpsertOperation::Add,
        field_number: 2,
        argument: 2,
    };

    println!("Upsert result: {:?}", tarantool_instance.request(&upsert).unwrap_or_else(&error_handler));

    let insert = Insert {
        space: 512,
        keys: &vec![Value::from(9)]
    };

    println!("Insert result: {:?}", tarantool_instance.request(&insert).unwrap_or_else(&error_handler));
}