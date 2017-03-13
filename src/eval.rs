use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;
use tarantool::Tarantool;

#[derive(Debug)]
pub struct Eval<'a> {
    expression: &'a str,
    keys: &'a Vec<Value>,
}

impl<'a> Eval<'a> {
    pub fn perform<I>(&self, state: &mut Tarantool)
                      -> Result<Value, String>
        where I: Serialize
    {
        let wrapped_keys = Value::Array(self.keys.clone());
        let keys_buffer = serialize_keys(wrapped_keys);
        let function_name = serialize_keys(Value::String(self.expression.into()));
        let request_id = state.get_id();
        let header = header(RequestTypeKey::Eval, request_id);
        let mut body = [&[0x82][..],
            &[Code::EXPR as u8][..],
            &function_name[..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]]
            .concat();
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

