use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;

#[derive(Debug, Builder)]
pub struct Eval<'a> {
    id: u32,
    expression: &'a str,
    keys: &'a Vec<Value>,
}

impl<'a> Eval<'a> {
    pub fn perform<I>(&self)
                      -> Result<Value, String>
        where I: Serialize
    {
        let wrapped_keys = Value::Array(keys);
        let keys_buffer = serialize_keys(wrapped_keys);
        let function_name = serialize_keys(Value::String(expression.into()));
        let request_id = self.get_id();
        let header = header(RequestTypeKey::Eval, request_id);
        let mut body = [&[0x82][..],
            &[Code::EXPR as u8][..],
            &function_name[..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]]
            .concat();
        let response = request(&header, &body);
        process_response(&response)
    }
}

