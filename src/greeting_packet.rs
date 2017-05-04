#[derive(Debug, Clone)]
pub struct GreetingPacket {
    pub greeting: String,
    pub salt: String,
}

impl GreetingPacket {
    pub fn new(salt: String, greeting: String) -> GreetingPacket {
        GreetingPacket {
            greeting: greeting,
            salt: salt,
        }
    }
}
