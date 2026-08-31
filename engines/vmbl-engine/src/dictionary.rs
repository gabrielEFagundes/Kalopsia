#[allow(non_camel_case_types, dead_code)]
pub enum Bytecode {
    // -headers
    MAGIC_NUM = 0xDEAF,
    VERSION = 0xA01,

    DEFINE = 0x01,
    QUERY = 0x02,
    NODE = 0x30,
    OBJ = 0x31,
    PATH = 0x32,
    NEXT = 0x34,

    PUSH = 0x50,
    MK_ARRAY = 0x70,
    DEAD = 0x00, // unreachable code
}

#[repr(u8)]
pub enum ByteTokenType {
    STRING = 0x11,
    INT = 0x22,
    DOUBLE = 0x33,
    DEAD = 0x00, // unreachable code
}

/// Just maps an `u8` to a `ByteTokenType`
pub fn as_bytetokentype(v: u8) -> ByteTokenType {
    match v {
        0x11 => ByteTokenType::STRING,
        0x22 => ByteTokenType::INT,
        0x33 => ByteTokenType::DOUBLE,
        _ => ByteTokenType::DEAD,
    }
}

pub fn as_bytecode(v: u8) -> Bytecode {
    match v {
        0x01 => Bytecode::DEFINE,
        0x02 => Bytecode::QUERY,
        0x30 => Bytecode::NODE,
        0x31 => Bytecode::OBJ,
        0x32 => Bytecode::PATH,
        0x34 => Bytecode::NEXT,
        0x50 => Bytecode::PUSH,
        0x70 => Bytecode::MK_ARRAY,
        _ => Bytecode::DEAD,
    }
}
