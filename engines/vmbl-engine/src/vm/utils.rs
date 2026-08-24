use crate::dictionary::Bytecode;

pub fn is_bytecode_valid(bytes: &[u8]) -> bool{
    let expect_magic_num = u16::from_be_bytes([bytes[1], bytes[0]]) == Bytecode::MAGIC_NUM as u16;
    let expect_version = u16::from_be_bytes([bytes[3], bytes[2]]) == Bytecode::VERSION as u16;
    // maybe add more verifiers, such as constants count, incase more safety is necessary.

    return expect_magic_num && expect_version;
}