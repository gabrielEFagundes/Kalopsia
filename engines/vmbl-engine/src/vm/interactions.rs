use std::collections::HashMap;

use knowledge_engine::{objects::{node::Node, obj::Obj}, runtime_utils::Graph};
use shared::data_types::ValueType;
use shared::orderedf64::Orderedf64;

use crate::vm::utils::{map_node_vals, map_obj_vals};

pub fn create_node(stack_arr: &mut Vec<ValueType>) -> Node {
    let mut mapped_vals: HashMap<String, ValueType> = HashMap::new();

    for _ in 0..stack_arr.len() {
        let value = stack_arr.pop();
        let key = stack_arr.pop();

        match key {
            Some(ValueType::Str(key)) => match key.as_str() {
                "name" | "difficulty" | "hours" | "req_skills" | "gain_skills" => {
                    mapped_vals.insert(key, value.unwrap());
                }

                _ => break,
            },

            None => break,

            _ => panic!(
                "[ERROR] cannot map node from script to Kalopsia's acceptable format\nmake sure all fields on VMBL are compatible with Kalopsia"
            ),
        }
    }

    let (name, difficulty, hours, req_skills, gain_skills) = map_node_vals(mapped_vals);
    Node::new(name, Orderedf64(difficulty), hours, req_skills, gain_skills)
}

#[allow(dead_code, unused)]
pub fn create_obj(stack_arr: &mut Vec<ValueType>) -> Obj {
    let mut mapped_vals: HashMap<String, ValueType> = HashMap::new();

    for _ in 0..stack_arr.len() {
        let value = stack_arr.pop();
        let key = stack_arr.pop();

        match key {
            Some(ValueType::Str(key)) => match key.as_str() {
                "skills" => {
                    mapped_vals.insert(key, value.unwrap());
                }
                _ => break,
            },

            None => break,
            _ => panic!(
                "[ERROR] cannot map obj from script to Kalopsia's acceptable format\nmake sure all fields on VMBL are compatible with Kalopsia"
            ),
        }
    }

    let skills = map_obj_vals(mapped_vals);
    Obj::new(skills)
}

pub fn query_node(stack_arr: &mut Vec<ValueType>, graph: &mut Graph) -> Node{
    for _ in 0..stack_arr.len(){
        let value = stack_arr.pop().unwrap();
        let key = stack_arr.pop();

        match key{
            Some(ValueType::Str(key)) => match key.as_str(){
                "id" => {
                    graph.nodes().iter().find(|p| p.id.0 == *value.as_ref_int());
                },
                "name" => {
                    graph.nodes().iter().find(|p| p.name == *value.as_ref_str());
                },
                "difficulty" => {
                    graph.nodes().iter().find(|p| p.difficulty.0 == *value.as_ref_double());
                },
                "hours" => {
                    graph.nodes().iter().find(|p| p.hours == *value.as_ref_int());
                },
                "req_skills" => {
                    graph.nodes().iter().find(|p| p.req_skills.contains(value.as_ref_str()));
                },
                "gain_skills" => {
                    graph.nodes().iter().find(|p| p.gain_skills.contains(value.as_ref_str()));
                },
                "interest" => {
                    graph.nodes().iter().find(|p| p.interest() == *value.as_ref_int());
                },
                "idea_added_at" => {
                    graph.nodes().iter().find(|p| p.idea_added_at().to_string().eq(value.as_ref_str())); // :|
                },
                "state" => {
                    graph.nodes().iter().find(|p| p.state() as i32 == *value.as_ref_int());
                },
                // search by connections in the future.
                _ => break
            },
            _ => panic!("[ERROR] unknown attribute parsed on QUERY\nmake sure all fields on VMBL are compatible with Kalopsia")
        }
    }

    

    todo!()
}