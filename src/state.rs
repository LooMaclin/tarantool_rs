use std::borrow::Cow;
use greeting_packet::GreetingPacket;

#[derive(Debug, Clone)]
pub struct State<'a> {
    pub address: Cow<'a, str>,
    pub user: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub greeting_packet: GreetingPacket<'a>,
    pub request_id: u64,
}

impl<'a> State<'a> {
    pub fn get_id(&mut self) -> u64 {
        self.request_id += 1;
        self.request_id
    }
}
