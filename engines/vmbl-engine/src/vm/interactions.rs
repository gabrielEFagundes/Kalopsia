use std::collections::{HashMap};

use knowledge_engine::{objects::{node::Node, obj::Obj}, runtime_utils::Graph};
use shared::{data_types::ValueType, debug};
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

pub fn query_node<'a>(stack_arr: &mut Vec<ValueType>, graph: &'a mut Graph) -> Vec<&'a Node>{
    let mut node_arr = Vec::new();
    for _ in 0..stack_arr.len(){
        let value = stack_arr.pop().unwrap_or_default();
        let key = stack_arr.pop();

        match key{
            Some(ValueType::Str(key)) => match key.as_str(){
                "id" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.id.0 == *value.as_ref_int()).expect("Node not found."));
                },
                "name" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.name == *value.as_ref_str()).expect("Node not found."));
                },
                "difficulty" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.difficulty.0 == *value.as_ref_double()).expect("Node not found."));
                },
                "hours" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.hours == *value.as_ref_int()).expect("Node not found."));
                },
                "req_skills" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.req_skills.contains(value.as_ref_str())).expect("Node not found."));
                },
                "gain_skills" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.gain_skills.contains(value.as_ref_str())).expect("Node not found."));
                },
                "interest" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.interest() == *value.as_ref_int()).expect("Node not found."));
                },
                "idea_added_at" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.idea_added_at().to_string().eq(value.as_ref_str())).expect("Node not found.")); // :|
                },
                "state" => {
                    node_arr.push(graph.nodes().iter().find(|p| p.state() as i32 == *value.as_ref_int()).expect("Node not found."));
                },
                // search by connections in the future.
                _ => break
            },
            None => break,
            _ => panic!("[ERROR] unknown attribute parsed on QUERY\nmake sure all fields on VMBL are compatible with Kalopsia")
        }
    }

    node_arr
}

pub fn query_obj<'a>(stack_arr: &mut Vec<ValueType>, graph: &'a mut Graph) -> Vec<&'a Obj>{
    let mut obj_arr = Vec::new();

    for _ in 0..stack_arr.len(){
        let value = stack_arr.pop().unwrap_or_default();
        let key = stack_arr.pop();

        match key{
            Some(ValueType::Str(key)) => match key.as_str(){
                // "nodes_done" => {
                //     obj_arr.push(graph.objects().iter().find(|p| p.nodes_done()));
                // },
                "skills" => {
                    obj_arr.push(graph.objects().iter().find(|p| p.skills().contains(value.as_ref_str())).expect("Object not found."));
                },
                _ => break
            },
            None => break,
            _ => panic!("[ERROR] unknown attribute parsed on QUERY\nmake sure all fields on VMBL are compatible with Kalopsia")
        }
    }
    
    obj_arr
}