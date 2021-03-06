# tarantool_rs
Sync/Async tarantool database connector.

[![Build Status](https://travis-ci.org/LooMaclin/tarantool_rs.svg?branch=master)](https://travis-ci.org/LooMaclin/tarantool_rs)

[![Coverage Status](https://coveralls.io/repos/github/LooMaclin/tarantool_rs/badge.svg?branch=master)](https://coveralls.io/github/LooMaclin/tarantool_rs?branch=master)

# Overview
- [Install](#install)
- [Usage](#usage)
    - [Include extern crate](#include-extern-crate)
    - [Use modules](#use-modules)
    - [Create tarantool connection instance](#create-tarantool-connection-instance)
    - [Retrieve space id](#retrieve-space-id)
    - [Retrieve index id](#retrieve-index-id)
    - [Select](#select)
    - [Insert](#insert)
    - [Delete](#delete)
    - [Upsert](#upsert)
    - [Replace](#replace)
    - [Call](#call)
    - [Eval](#eval)
    - [Update common](#update-common)
    - [Update string](#update-string)
    - [Update integer](#update-integer)

# Install

```toml
[dependencies]
tarantool = { git = "https://github.com/LooMaclin/tarantool_rs.git" }
```

# Usage

## Include extern crate 

```rust

extern crate tarantool;

```

## Use modules

```rust

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation, Upsert,
                UpsertOperation};

```

## Create tarantool connection instance

```rust

    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });

    let error_handler = |err| panic!(SyncClient, err);

```

## Retrieve space id

```rust

    let space_id = tarantool_instance.fetch_space_id("tester");
    debug!("Tester space id: {}", space_id);

```

## Retrieve index id

```rust

    let index_id = tarantool_instance.fetch_index_id(space_id, "primary");
    debug!("Tester primary index id: {}", index_id);

```

## Select

```rust

    let select = Select {
        space: 512,
        index: 0,
        limit: 100,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    };

    let tuples = tarantool_instance.request(&select).unwrap_or_else(&error_handler);

    debug!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        debug!("{}: {:?}", index, tuple);
    }
    
```

## Insert

```rust

    let insert = Insert {
        space: 512,
        keys: &vec![Value::from(9)]
    };

    debug!("Insert result: {:?}", tarantool_instance.request(&insert).unwrap_or_else(&error_handler));

```

## Replace

```rust

    let replace = Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    };

    debug!("Replace result: {:?}", tarantool_instance.request(&replace).unwrap_or_else(&error_handler));

```

## Update integer

```rust

    let update_integer = UpdateInteger {
        space: 512,
        index: 0,
        operation_type: IntegerOperation::Addition,
        field_number: 2,
        argument: 1,
        keys: &vec![Value::from(4)]
    };

    debug!("Integer-Update result: {:?}", tarantool_instance.request(&update_integer).unwrap_or_else(&error_handler));

```

## Update string

```rust

    let update_string = UpdateString {
        space: 512,
        index: 0,
        field_number: 1,
        position: 3,
        offset: 3,
        argument: "TEST UPDATE STRING".into(),
        keys: &vec![Value::from(2)]
    };

    debug!("String-Update result: {:?}", tarantool_instance.request(&update_string).unwrap_or_else(&error_handler));

```

## Update common

```rust

    let update_common = UpdateCommon {
        space: 512,
        index: 0,
        operation_type: CommonOperation::Assign,
        field_number: 3,
        argument: Value::String(String::from("Test Update Common Assign")),
        keys: &vec![Value::from(6)]
    };

    debug!("Common-Update result: {:?}", tarantool_instance.request(&update_common).unwrap_or_else(&error_handler));

```

## Delete

```rust

    let delete = Delete {
        space: 512,
        index: 0,
        keys: &vec![Value::from(3)]
    };

    debug!("Delete result: {:?}", tarantool_instance.request(&delete).unwrap_or_else(&error_handler));

```

## Call

```rust

    let call = Call {
        function_name: "test",
        keys: &vec![]
    };

    debug!("Call result: {:?}", tarantool_instance.request(&call).unwrap_or_else(&error_handler));

```

## Eval

```rust

    let eval = Eval {
        expression: r#"return 5+5"#,
        keys: &vec![]
    };

    debug!("Eval result: {:?}", tarantool_instance.request(&eval).unwrap_or_else(&error_handler));

```

## Upsert

```rust

    let upsert = Upsert {
        space: 512,
        keys: &vec![Value::from(5)],
        operation_type: UpsertOperation::Add,
        field_number: 2,
        argument: 2,
    };

    debug!("Upsert result: {:?}", tarantool_instance.request(&upsert).unwrap_or_else(&error_handler));

```


#Roadmap

- [ ] Without heap-allocation
- [ ] Sync connector
- [ ] Ergonomic API with builders
- [ ] Async connector
- [ ] Full test coverage
- [ ] Full-application examples
