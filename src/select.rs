use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use byteorder::ByteOrder;
use serde::Serialize;
use std::net::TcpStream;
use std::io::{Read, Write};
use tarantool::Tarantool;

#[derive(Debug, Builder)]
pub struct Select<'a> {
    space: u16,
    index: u8,
    limit: u8,
    offset: u8,
    iterator: IteratorType,
    keys: &'a Vec<Value>
}

impl<'a> Select<'a> {

    pub fn perform(&self, mut state: &mut Tarantool)
                     -> Result<Value, String>
    {
        let keys_buffer = serialize_keys(self.keys);
        let header = header(RequestTypeKey::Select, state.get_id());
        let mut body = [&[0x86][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[self.index][..],
            &[Code::Limit as u8][..],
            &[self.limit][..],
            &[Code::Offset as u8][..],
            &[self.offset][..],
            &[Code::Iterator as u8][..],
            &[self.iterator as u8][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], self.space);
        let response = request(&header, &body, &mut state.descriptor);
        process_response(&response)
    }
}

