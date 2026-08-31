use crate::objects::node::Node;

/// Main struct for defining an external object (not part of the graph).
///
/// Used mostly for defining the person, in case of Kalopsia's v1.
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Obj {
    nodes_done: Vec<Node>,
    skills: Vec<String>, //currConfidence: HashMap<Node, i32> I need to study the ways I can do this
}

impl Obj {
    /// Constructor to create a new Dev profile
    ///
    /// ### Default fields:
    /// - `nodesDone`: Always starts empty and gets added dynamically
    pub fn new(skills: Vec<String>) -> Self {
        Self {
            nodes_done: Vec::new(),
            skills,
        }
    }

    pub fn from(nodes_done: Vec<Node>, skills: Vec<String>) -> Self {
        Self { nodes_done, skills }
    }

    /// Adds a node to `Obj`
    pub fn add_node(&mut self, node: Node) {
        self.nodes_done.push(node);
    }

    /// Adds multiple nodes at once to `Dev`
    pub fn add_nodes(&mut self, new_node: Vec<&Node>) {
        for node in new_node {
            self.add_node(node.clone());
        }
    }

    pub fn nodes_done(&self) -> &Vec<Node> {
        &self.nodes_done
    }

    pub fn skills(&self) -> &Vec<String> {
        &self.skills
    }
}
