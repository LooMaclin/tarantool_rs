use client::Client;
use tarantool::Tarantool;

impl<'a> Client<String> for Tarantool<'a> {
    fn connect(&self) -> String {
        unimplemented!()
    }

    fn select(&self) -> String {
        unimplemented!()
    }

    fn insert(&self) -> String {
        unimplemented!()
    }

    fn replace(&self) -> String {
        unimplemented!()
    }

    fn update(&self) -> String {
        unimplemented!()
    }

    fn delete(&self) -> String {
        unimplemented!()
    }

    fn call_16(&self) -> String {
        unimplemented!()
    }

    fn eval(&self) -> String {
        unimplemented!()
    }

    fn upsert(&self) -> String {
        unimplemented!()
    }

    fn call(&self) -> String {
        unimplemented!()
    }
}