use std::{collections::{HashMap}};
use std::collections::hash_map::Keys;
use std::collections::{BTreeMap, HashSet};
use chrono::{DateTime, Local};

use crate::{enumerations::State, objects::edge::Edge};

/// Main struct used to define a Node for the graph
#[allow(dead_code, non_snake_case)]
#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Node{
    pub name: String,
    pub difficulty: i32,
    pub hours: i32,
    pub reqSkills: Vec<String>,
    pub gainSkills: Vec<String>,
    interest: i32,
    ideaAddedAt: DateTime<Local>,
    state: State,
    connections: Vec<BTreeMap<Node, Edge>>
}

impl Node{
    /// Constructor with the required fields to create a node.
    ///
    /// Default fields:
    /// - interest: Starts out as 100 and decays according to the Dev currentEntropy value
    /// - ideaAddedAt: DateTime according to the host's current datetime when a Node is created
    /// - state: Always defaults to NOT_STARTED
    /// - connections: Always starts empty and is dynamically allocated
    pub fn new(name: String, difficulty: i32, hours: i32, req_skills: Vec<String>, gain_skills: Vec<String>) -> Self{
        Self {  name,
                difficulty,
                hours,
                reqSkills: req_skills, 
                gainSkills: gain_skills, 
                interest: 100, 
                ideaAddedAt: Local::now(), // now.
                state: State::NOT_STARTED, // starts as NOT_STARTED
                connections: Vec::new() // will not have connections when initializing
        }
    }

    /// Function used to add new connections to the Node, including other Nodes and the edges that connects them
    ///
    /// Updates both the current Node and the connected Node
    pub fn add_conn(&mut self, conn: BTreeMap<Node, Edge>) {
        // TODO: fix the performance of this code, holy shit
        let usable_connection = conn.clone();

        let connects_to: Vec<Node> = usable_connection.keys().cloned().collect();
        for mut conn in connects_to{
            for test in self.connections.iter(){
                conn.connections.push(test.clone());
            }
        }

        self.connections.push(usable_connection);
    }

    pub fn update_state(&mut self, new_state: State){
        self.state = new_state;
    }

    // maybe an interest decay function?
}