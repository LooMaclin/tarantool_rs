#[derive(Debug, PartialEq, RustcEncodable)]
pub enum ProtocolParts {
    StringPart(String),
    StringArray(Vec<String>),
    U32(u32),
    U8(u8)
}