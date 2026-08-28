use std::collections::HashMap;

use knowledge_engine::objects::{node::Node, obj::Obj};
use shared::orderedf64::Orderedf64;
use shared::data_types::ValueType;

use crate::vm::utils::{map_node_vals, map_obj_vals};

pub fn create_node(stack_arr: &mut Vec<ValueType>) -> Node{
    let mut mapped_vals: HashMap<String, ValueType> = HashMap::new();

    for _ in 0..stack_arr.len(){
        let value = stack_arr.pop();
        let key = stack_arr.pop();

        match key{
            Some(ValueType::Str(key)) => match key.as_str() {
                "name" | "difficulty" | "hours" | "req_skills" | "gain_skills" 
                    => { mapped_vals.insert(key, value.unwrap()); }
                
                _ => break
            }

            None => break,

            _ => panic!("[ERROR] cannot map node from script to Kalopsia's acceptable format\nmake sure all fields on VMBL are compatible with Kalopsia")
        }
    }

    let (name, difficulty, hours, req_skills, gain_skills) = map_node_vals(mapped_vals);
    return Node::new(name, Orderedf64(difficulty), hours, req_skills, gain_skills);
}

#[allow(dead_code, unused)]
pub fn create_obj(stack_arr: &mut Vec<ValueType>) -> Obj{
    let mut mapped_vals: HashMap<String, ValueType> = HashMap::new();

    for _ in 0..stack_arr.len(){
        let value = stack_arr.pop();
        let key = stack_arr.pop();

        match key{
            Some(ValueType::Str(key)) => match key.as_str() {
                "skills" => { mapped_vals.insert(key, value.unwrap()); }
                _ => break
            }

            None => break,
            _ => panic!("[ERROR] cannot map obj from script to Kalopsia's acceptable format\nmake sure all fields on VMBL are compatible with Kalopsia")
        }
    }

    let skills = map_obj_vals(mapped_vals);
    return Obj::new(skills);
}