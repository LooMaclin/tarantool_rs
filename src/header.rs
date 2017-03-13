#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub code: u32,
    pub sync: u64,
    pub schema_id: u32,
}