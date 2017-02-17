#![feature(test)]
extern crate test;
extern crate tarantool;

use tarantool::tarantool::Tarantool;
use test::Bencher;

    #[bench]
    fn tarantool_select(b: &mut Bencher) {
        let mut tarantool_instance = Tarantool::auth("127.0.0.1:3301", "test", "test").unwrap_or_else(|err| {
            panic!("Tarantool auth error: {:?}", &err);
        });
        b.iter(|| {
            let tuples = tarantool_instance.select(512, 0, 10, 0, IteratorType::All, (3)).unwrap_or_else(|err| {
                panic!("Tarantool select error: {:?}", &err);
            });
        });
    }