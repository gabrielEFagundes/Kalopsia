use std::{collections::HashMap, time::Instant};

use crate::enumerations::{Relationship, State};

#[allow(dead_code)]
pub struct Edge{
    weight: f64,
    relationship: Relationship
}

#[allow(dead_code, non_snake_case)]
pub struct Node{
    name: &'static str, // maybe change to String because requires unsafe blocks
    difficulty: i32,
    hours: i32,
    reqSkills: Vec<String>,
    gainSkills: Vec<String>,
    interest: i32,
    ideaAddedAt: Instant,
    state: State,
    connections: Vec<HashMap<Node, Edge>>
}

#[allow(dead_code, non_snake_case)]
pub struct Dev{
    nodesDone: Vec<Node>,
    skills: Vec<String>
    //currConfidence: HashMap<Node, i32>
}