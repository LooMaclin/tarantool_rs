# tarantool_rs
Sync/Async tarantool database connector.

[![Build Status](https://travis-ci.org/LooMaclin/tarantool_rs.svg?branch=master)](https://travis-ci.org/LooMaclin/tarantool_rs)

[![Coverage Status]
(https://coveralls.io/repos/github/LooMaclin/tarantool_rs/badge.svg?branch=master)](https://coveralls.io/github/LooMaclin/tarantool_rs?branch=master)

#Install

```toml
[dependencies]
tarantool = { git = "https://github.com/LooMaclin/tarantool_rs.git" }
```

#Usage

#Include extern crate 
```rust
extern crate tarantool;


```

#Use modules

```

use tarantool::{Value, Tarantool, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,CommonOperation};

```

#Create tarantool connection instance

```

    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("err: {}", err);
    });
    let error_handler = |err| panic!("Tarantool error: {}", err);

```

#Select

```rust

    let tuples = Select {
        space: 512,
        index: 0,
        limit: 10,
        offset: 0,
        iterator: IteratorType::All,
        keys: &vec![]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler);
    println!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
    
```

#Insert

```rust

    println!("Insert result: {:?}",
    Insert {
        space: 512,
        keys: &vec![Value::from(9)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));

```

#Replace

```rust

    println!("Replace result: {:?}", Replace {
        space: 512,
        keys: &vec![Value::from(1), Value::String(String::from("TEST REPLACE"))]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));

```

#Update integer

```rust

use tarantool::tarantool::Tarantool;
use tarantool::operation::IntegerOperation;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_integer(512, 0, (3), IntegerOperation::Addition, 2, 5).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}

```

#Update string

```rust

use tarantool::tarantool::Tarantool;
use tarantool::operation::StringOperation;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_string(512, 0, (3), 1, 2, 2, "FUCK").unwrap_or_else(|err| {
        panic!("Tarantool update string error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}

```

#Update common

```rust

    println!("Common-Update result: {:?}", UpdateCommon {
        space: 512,
        index: 0,
        operation_type: CommonOperation::Assign,
        field_number: 3,
        argument: Value::String(String::from("Test Update Common Assign")),
        keys: &vec![Value::from(6)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));

```

#Delete

```rust

    println!("Delete result: {:?}", Delete {
        space: 512,
        index: 0,
        keys: &vec![Value::from(3)]
    }
        .perform(&mut tarantool_instance)
        .unwrap_or_else(&error_handler));

```

#Call

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let function_argument = vec![Value::from(12)];
    let result = tarantool_instance.call("test", function_argument).unwrap_or_else(|err| {
        panic!("Tarantool call error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

```

#Call 16

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let function_argument = vec![Value::from(12)];
    let result = tarantool_instance.call_16("test", function_argument).unwrap_or_else(|err| {
        panic!("Tarantool call 16 error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

```

#Eval

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let function_argument = vec![];
    let result = tarantool_instance.eval("test()", function_argument).unwrap_or_else(|err| {
        panic!("Tarantool eval error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

```

#Upsert

```rust

use tarantool::tarantool::Tarantool;
use tarantool::operation::UpsertOperation;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.upsert(512, (3), UpsertOperation::Add, 2, 5).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}

```


#Roadmap

- [ ] Without heap-allocation
- [ ] Sync connector
- [ ] Ergonomic API with builders
- [ ] Async connector
- [ ] Full test coverage
- [ ] Full-application examples
