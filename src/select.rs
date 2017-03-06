use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request};

#[derive(Debug, Builder)]
pub struct Select<'a> {
    id: u32,
    space: u16,
    index: u8,
    limit: u8,
    offset: u8,
    iterator: &'a IteratorType,
    keys: &'a Vec<Value>,
}

impl Select {
    pub fn perform<I>(&self)
                     -> Result<Value, String>
        where I: Serialize
    {
        let keys_buffer = Tarantool::serialize_keys(keys);
        let header = header(RequestTypeKey::Select, self.id);
        let mut body = [&[0x86][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::IndexId as u8][..],
            &[index][..],
            &[Code::Limit as u8][..],
            &[limit][..],
            &[Code::Offset as u8][..],
            &[offset][..],
            &[Code::Iterator as u8][..],
            &[iterator as u8][..],
            &[Code::Key as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = request(&header, &body);
        Tarantool::process_response(&response)
    }
}

