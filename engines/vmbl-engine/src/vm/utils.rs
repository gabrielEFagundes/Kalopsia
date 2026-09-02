use crate::dictionary::Bytecode;
use shared::data_types::ValueType;
use std::collections::{HashMap, HashSet};

pub fn is_bytecode_valid(bytes: &[u8]) -> bool {
    let expect_magic_num = u16::from_be_bytes([bytes[1], bytes[0]]) == Bytecode::MAGIC_NUM as u16;
    let expect_version = u16::from_be_bytes([bytes[3], bytes[2]]) == Bytecode::VERSION as u16;
    // maybe add more verifiers, such as constants count, incase more safety is necessary.

    expect_magic_num && expect_version
}

pub fn map_node_vals(
    mut stack: HashMap<String, ValueType>,
) -> (String, f64, i32, HashSet<String>, HashSet<String>) {
    let name = stack.remove("name").unwrap().as_str();
    let difficulty = stack.remove("difficulty").unwrap().as_double();
    let hours = stack.remove("hours").unwrap().as_int();
    let req_skills = stack.remove("req_skills").unwrap().as_hashset();
    let gain_skills = stack.remove("gain_skills").unwrap().as_hashset();

    (
        name,
        difficulty,
        hours,
        req_skills.into_iter().map(|v| v.as_str()).collect(),
        gain_skills.into_iter().map(|v| v.as_str()).collect(),
    )
}

pub fn map_obj_vals(mut stack: HashMap<String, ValueType>) -> HashSet<String> {
    stack
        .remove("skills")
        .unwrap()
        .as_hashset()
        .into_iter()
        .map(|v| v.as_str())
        .collect()
}
