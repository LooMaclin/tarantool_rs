#[macro_use]
extern crate log;
extern crate tarantool;

use tarantool::{Value, SyncClient, IteratorType, Select, Insert, Replace, Delete, UpdateCommon,
                CommonOperation, Call, Eval, UpdateString, UpdateInteger, IntegerOperation,
                Upsert, UpsertOperation};

#[derive(Debug)]
pub struct User {
    pub login: String,
    pub password: String,
    pub likes: u64,
    pub posts: u64,
}

fn main() {

    let mut tarantool_instance = SyncClient::auth("127.0.0.1:3301", "test", "test")
        .unwrap_or_else(|err| {
                            panic!("err: {}", err);
                        });

    let error_handler = |err| panic!("Tarantool error: {}", err);

    let users = vec![User {
                         login: "user_1".into(),
                         password: "123".into(),
                         likes: 11,
                         posts: 25,
                     },
                     User {
                         login: "user_2".into(),
                         password: "12345".into(),
                         likes: 22,
                         posts: 1,
                     },
                     User {
                         login: "user_3".into(),
                         password: "1664".into(),
                         likes: 0,
                         posts: 277,
                     }];

    for (index, user) in users.iter().enumerate() {
        let insert = Insert {
            space: 512,
            keys: vec![Value::from(index),
                       Value::from(user.login.clone()),
                       Value::from(user.password.clone()),
                       Value::from(user.likes),
                       Value::from(user.posts)],
        };
        debug!("Insert result: {:?}",
                 tarantool_instance
                     .request(&insert)
                     .unwrap_or_else(&error_handler));
    }

    let select = Select {
        space: 512,
        index: 0,
        limit: 10,
        offset: 0,
        iterator: IteratorType::All,
        keys: vec![],
    };

    let tuples = tarantool_instance
        .request(&select)
        .unwrap_or_else(&error_handler);

    debug!("Select result: ");
    for (index, tuple) in tuples.as_array().unwrap().iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        debug!("{}: {:?}", index, tuple);
        debug!("{}: {:?}",
                 index,
                 User {
                     login: tuple[1].as_str().unwrap().into(),
                     password: tuple[2].as_str().unwrap().into(),
                     likes: tuple[3].as_u64().unwrap(),
                     posts: tuple[4].as_u64().unwrap(),
                 });

        let delete = Delete {
            space: 512,
            index: 0,
            keys: vec![tuple[0].clone()],
        };

        debug!("Delete result: {:?}",
                 tarantool_instance
                     .request(&delete)
                     .unwrap_or_else(&error_handler));
    }

}
