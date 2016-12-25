pub trait Client<T> {
    fn connect(&self) -> T;
    fn select(&self) -> T;
    fn insert(&self) -> T;
    fn replace(&self) -> T;
    fn update(&self) -> T;
    fn delete(&self) -> T;
    fn call_16(&self) -> T;
    fn eval(&self) -> T;
    fn upsert(&self) -> T;
    fn call(&self) -> T;
}