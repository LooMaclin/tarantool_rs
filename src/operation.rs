pub enum IntegerOperation {
    Addition = 0x2B,
    Subtraction = 0x2D,
    And = 0x26,
    Xor = 0x5E,
    Or = 0x7C,
}

pub enum CommonOperation {
    Delete = 0x23,
    Insert = 0x21,
    Assign = 0x3D,
}

pub enum StringOperation {
    Splice = 0x3A,
}

pub enum UpsertOperation {
    Add = 0x2B,
    Subtract = 0x2D,
    Assign = 0x3D,
    Insert = 0x21,
    Delete = 0x23,
}

pub const FIX_STR_PREFIX: u8 = 0xA1;
