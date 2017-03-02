# tarantool_rs
Sync/Async tarantool database connector.

[![Build Status](https://travis-ci.org/LooMaclin/tarantool_rs.svg?branch=master)](https://travis-ci.org/LooMaclin/tarantool_rs)

#Install

```toml
[dependencies]
tarantool = { git = "https://github.com/LooMaclin/tarantool_rs.git" }
```

#Usage

```rust
extern crate tarantool;
```

#Select

```rust


use tarantool::tarantool::Tarantool;
use tarantool::iterator_type::IteratorType;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.select(512, 0, 10, 0, IteratorType::All, (3)).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
}

```

#Insert

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let inserting_value = vec![Value::from(11), Value::String("Black Room".to_string()), Value::from(2017)];
    let result = tarantool_instance.insert(512, inserting_value).unwrap_or_else(|err| {
        panic!("Tarantool insert error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

```

#Replace

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let inserting_value = vec![Value::from(6), Value::String("Black Some Sign".to_string()), Value::from(2005)];
    let result = tarantool_instance.replace(512, inserting_value).unwrap_or_else(|err| {
        panic!("Tarantool insert error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

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

use tarantool::tarantool::Tarantool;
use tarantool::operation::CommonOperation;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.update_common(512, 0, (3), CommonOperation::Assign, 2, Value::from(2015)).unwrap_or_else(|err| {
        panic!("Tarantool update common error: {:?}", &err);
    });
    println!("Result: {:?}", tuples);
}

```

#Delete

```rust

use tarantool::tarantool::Tarantool;
use tarantool::Value;

fn main() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let delete_key = vec![Value::from(12)];
    let result = tarantool_instance.delete(512, 0, delete_key).unwrap_or_else(|err| {
        panic!("Tarantool delete error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

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
