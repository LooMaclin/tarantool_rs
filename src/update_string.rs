use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;

#[derive(Debug, Builder)]
pub struct UpdateString<'a> {
    id: u32,
    space: u16,
    index: u8,
    field_number: u8,
    position: u8,
    offset: u8,
    argument: Cow<'a,str>,
    keys: &'a Vec<Value>,
}

impl<'a> UpdateString<'a> {
    pub fn perform<I>(&self)
                      -> Result<Value, String>
        where I: Serialize
    {
        let keys_buffer = serialize_keys(keys);
        let request_id = self.get_id();
        let header = header(RequestTypeKey::Update, request_id);
        let wrapped_argument = Value::String(argument.into().into_owned());
        let mut serialized_argument = Vec::new();
        wrapped_argument.serialize(&mut Serializer::new(&mut serialized_argument)).unwrap();
        let mut body = [&[0x84][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Key as u8][..],
            &keys_buffer[..],
            &[Code::Tuple as u8][..],
            &[0x91,
                0x95,
                FIX_STR_PREFIX,
                StringOperation::Splice as u8,
                field_number,
                position,
                offset][..],
            &serialized_argument[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = request(&header, &body);
        process_response(&response)
    }
}
