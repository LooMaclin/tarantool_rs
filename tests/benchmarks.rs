#![feature(test)]
extern crate test;

extern crate tarantool;
extern crate rmpv;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

use tarantool::tarantool::Tarantool;
use tarantool::client::Client;
use tarantool::iterator_type::IteratorType;
use std::borrow::Cow;
use rmpv::Value;

use tarantool::async_client::TarantoolAsyncClient;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;
use test::Bencher;

#[bench]
fn tarantool_select_benchmark(b: &mut Bencher) {
    let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
        panic!("Tarantool auth error: {:?}", &err);
    });
    b.iter(|| {
        tarantool_instance.select(512, 0, 10, 0, IteratorType::All, (3)).unwrap();
    });
}