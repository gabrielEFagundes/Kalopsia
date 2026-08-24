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