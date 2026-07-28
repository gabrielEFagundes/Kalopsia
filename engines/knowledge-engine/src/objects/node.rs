use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::{enumerations::State, objects::edge::Edge};

/// Main struct used to define a Node for the graph
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Node {
    pub name: String, // maybe change to String because requires unsafe blocks
    pub difficulty: i32,
    pub hours: i32,
    pub reqSkills: Vec<String>,
    pub gainSkills: Vec<String>,
    interest: i32,
    ideaAddedAt: DateTime<Local>,
    state: State,
    connections: Vec<HashMap<Node, Edge>>,
}

impl Node {
    /// Constructor with the required fields to create a node.
    ///
    /// Default fields:
    /// - interest: Starts out as 100 and decays according to the Dev currentEntropy value
    /// - ideaAddedAt: DateTime according to the host's current datetime when a Node is created
    /// - state: Always defaults to NOT_STARTED
    /// - connections: Always starts empty and is dynamically allocated
    pub fn new(
        name: String,
        difficulty: i32,
        hours: i32,
        req_skills: Vec<String>,
        gain_skills: Vec<String>,
    ) -> Self {
        Self {
            name,
            difficulty,
            hours,
            reqSkills: req_skills,
            gainSkills: gain_skills,
            interest: 100,
            ideaAddedAt: Local::now(), // now.
            state: State::NOT_STARTED, // starts as NOT_STARTED
            connections: Vec::new(),   // will not have connections when initializing
        }
    }
}
