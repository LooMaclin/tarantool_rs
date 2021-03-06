
#[derive(Debug, Copy, Clone)]
pub enum IteratorType {
    Eq,
    Req,
    All,
    Lt,
    Le,
    Ge,
    Gt,
    BitsAllSet,
    BitsAnySet,
    BitsAllNotSet,
}
