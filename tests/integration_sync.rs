#![feature(test)]
extern crate test;

extern crate tarantool;
extern crate rmpv;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate serde;


use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::iterator_type::IteratorType;
use std::borrow::Cow;
use rmpv::Value;
use serde::Serialize;
use tarantool::async_client::TarantoolAsyncClient;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;
use test::Bencher;

#[test]
fn tarantool_sync_select() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let tuples = tarantool_instance.select(512, 0, 10, 0, IteratorType::All, (3)).unwrap_or_else(|err| {
        panic!("Tarantool select error: {:?}", &err);
    });
    for (index, tuple) in tuples.iter().enumerate() {
        let tuple = tuple.as_array().unwrap();
        println!("{}: {:?}", index, tuple);
    }
}

#[test]
fn tarantool_insert_select() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let inserting_value = vec![Value::from(6), Value::String("Black Room".to_string()), Value::from(2017)];
    let result = tarantool_instance.insert(512, inserting_value).unwrap_or_else(|err| {
        panic!("Tarantool insert error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}

#[test]
fn tarantool_replace_select() {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    let inserting_value = vec![Value::from(6), Value::String("Black Room Sign".to_string()), Value::from(2005)];
    let result = tarantool_instance.replace(512, inserting_value).unwrap_or_else(|err| {
        panic!("Tarantool insert error: {:?}", &err);
    });
    println!("Result: {:?}", result);
}


#[test]
#[ignore]
fn tarantool_async() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:3301".parse().unwrap();
    let handle = core.handle();

    core.run(
        TarantoolAsyncClient::connect(&addr, &handle)
            .and_then(|client| {
                client.call(vec![0])
                    .and_then(move |response| {
                        println!("CLIENT: {:?}", response);
                        Ok(())
                    })
            })
    ).unwrap();

}

