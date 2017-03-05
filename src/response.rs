use header::Header;

#[derive(Debug, Clone)]
pub struct Response {
    pub header: Header,
    pub body: Option<Vec<u8>>,
}
