#[allow(non_camel_case_types, dead_code)]
pub enum Bytecode{
    // -headers
    MAGIC_NUM = 0xDEAF,
    VERSION = 0xA01,

    DEFINE = 0x01,
    QUERY = 0x02,
    NODE = 0x30,
    OBJ = 0x31,
    PATH = 0x32,
    NEXT = 0x33,

    STRING = 0x11,
    INT = 0x22,
    DOUBLE = 0x44,

    PUSH = 0x50,
    MK_ARRAY = 0x70,
}

#[derive(Debug)]
pub enum ValueType{
    Str(String),
    Int(i32),
    Double(f64)
}

#[repr(u8)]
pub enum ByteTokenType{
    STRING = 0x11,
    INT = 0x22,
    DOUBLE = 0x33,
    DEAD = 0x00
}

/// Just maps an `u8` to a `ByteTokenType`
pub fn as_bytetokentype(v: u8) -> ByteTokenType{
    match v {
        0x11 => ByteTokenType::STRING,
        0x22 => ByteTokenType::INT,
        0x33 => ByteTokenType::DOUBLE,
        _ => ByteTokenType::DEAD
    }
}