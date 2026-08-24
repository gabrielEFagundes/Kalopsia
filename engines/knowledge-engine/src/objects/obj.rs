use crate::objects::node::Node;

/// Main struct for defining the profile of the developer (or person in the future)
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct Dev {
    nodesDone: Vec<Node>,
    skills: Vec<String>, //currConfidence: HashMap<Node, i32> I need to study the ways I can do this
}

impl Dev {
    /// Constructor to create a new Dev profile
    ///
    /// ### Default fields:
    /// - `nodesDone`: Always starts empty and gets added dynamically
    pub fn new(skills: Vec<String>) -> Self {
        Self {
            nodesDone: Vec::new(),
            skills,
        }
    }

    /// Adds a node to `Dev`
    pub fn add_node(&mut self, node: Node) {
        self.nodesDone.push(node);
    }

    /// Adds multiple nodes at once to `Dev`
    pub fn add_nodes(&mut self, new_node: Vec<&Node>) {
        for node in new_node {
            self.add_node(node.clone());
        }
    }
}
