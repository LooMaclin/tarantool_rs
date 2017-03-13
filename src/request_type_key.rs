#[derive(Copy, Debug, Clone)]
pub enum RequestTypeKey {
    // Command is successful
    Ok = 0x00,
    // User command codes
    Select = 0x01,
    Insert = 0x02,
    Replace = 0x03,
    Update = 0x04,
    Delete = 0x05,
    Call = 0x06,
    Auth = 0x07,
    Eval = 0x08,
    Upsert = 0x09,
    TypeStatMax = 0x0b,
    // Admin command codes
    Ping = 0x40,
    Join = 0x41,
    Subscribe = 0x42,
    TypeAdminMax = 0x43,
    TypeError = 0x8000,
}
