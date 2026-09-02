use crate::{enumerations::State, objects::edge::Edge};
use chrono::{DateTime, Local};
use lib::{id_engine::builder::Identifier, impl_identifier};
use lib::id_engine::builder::{HasId, IdPool};
use shared::orderedf64::Orderedf64;
use std::collections::{HashMap, HashSet};

/// Main struct used to define a Node for the graph
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: Identifier,
    pub name: String,
    pub difficulty: Orderedf64,
    pub hours: i32,
    pub req_skills: HashSet<String>,
    pub gain_skills: HashSet<String>,
    interest: i32,
    idea_added_at: DateTime<Local>,
    state: State,
    connections: Vec<HashMap<Identifier, Edge>>,
}

impl_identifier!(Node);
impl Node {
    /// Constructor with the required fields to create a node.
    ///
    /// ### Default fields:
    /// - `interest`: Starts out as 100 and decays according to the Dev currentEntropy value
    /// - `ideaAddedAt`: DateTime according to the host's current datetime when a Node is created
    /// - `state`: Always defaults to NOT_STARTED
    /// - `connections`: Always starts empty and is dynamically allocated
    pub fn new(
        name: String,
        difficulty: Orderedf64,
        hours: i32,
        req_skills: HashSet<String>,
        gain_skills: HashSet<String>,
    ) -> Self {
        Self {
            id: Self::alloc_id(),
            name,
            difficulty,
            hours,
            req_skills,
            gain_skills,
            interest: 100,
            idea_added_at: Local::now(), // now.
            state: State::NOT_STARTED,   // starts as NOT_STARTED
            connections: Vec::new(),     // will not have connections when initializing
        }
    }

    pub fn from(
        id: Identifier,
        name: String,
        difficulty: Orderedf64,
        hours: i32,
        req_skills: HashSet<String>,
        gain_skills: HashSet<String>,
        interest: i32,
        idea_added_at: DateTime<Local>,
        state: State,
        connections: Vec<HashMap<Identifier, Edge>>,
    ) -> Self {
        Self {
            id,
            name,
            difficulty,
            hours,
            req_skills,
            gain_skills,
            interest,
            idea_added_at,
            state,
            connections,
        }
    }

    /// Function used to add new connections to the Node, including other Nodes and the edges that connects them
    ///
    /// Updates both the current Node and the connected Node
    /// 
    /// # Attention
    /// Edges are CLONED when being inserted into the nodes' connections
    /// 
    /// This means that, if anywhere in the runtime of Kalopsia, those are changed, these cloned values WILL NOT
    /// change, nothing will break, but data will be wrong. 
    /// 
    /// Worth noting it here before future-dev Gabriel forgets and goes through another session of hellish debbuging.
    pub fn add_conn(&mut self, node: &mut Node, edge: Edge) {
        let mut conn: HashMap<Identifier, Edge> = HashMap::new();
        conn.insert(node.clone().id, edge.clone());

        self.connections.push(conn);

        let mut parent_connection = HashMap::new();
        parent_connection.insert(self.clone().id, edge);

        node.connections.push(parent_connection);
    }

    /// Updates the state of a Node. Utilitary method
    pub fn update_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    /// Getter for read-only access to interest field.
    pub fn interest(&self) -> i32 {
        self.interest
    }

    /// Getter for read-only access to ideaAddedAt field.
    pub fn idea_added_at(&self) -> DateTime<Local> {
        self.idea_added_at
    }

    /// Getter for read-only access to state field.
    pub fn state(&self) -> State {
        self.state
    }

    /// Getter for read-only access to connections field.
    pub fn connections(&self) -> &Vec<HashMap<Identifier, Edge>> {
        &self.connections
    }

    // maybe an interest decay function?
}
