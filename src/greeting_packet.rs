use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct GreetingPacket<'a> {
    pub greeting: Cow<'a, str>,
    pub salt: Cow<'a, str>,
}

impl<'a> GreetingPacket<'a> {
    pub fn new<S>(salt: S, greeting: S) -> GreetingPacket<'a>
        where S: Into<Cow<'a, str>>
    {
        GreetingPacket {
            greeting: greeting.into(),
            salt: salt.into(),
        }
    }

    pub fn greeting<S>(&'a mut self, greeting: S) -> &'a mut GreetingPacket
        where S: Into<Cow<'a, str>>
    {
        self.greeting = greeting.into();
        self
    }

    pub fn salt<S>(&'a mut self, salt: S) -> &'a mut GreetingPacket
        where S: Into<Cow<'a, str>>
    {
        self.salt = salt.into();
        self
    }
}
