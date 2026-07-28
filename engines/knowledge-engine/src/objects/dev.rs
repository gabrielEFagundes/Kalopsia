use crate::objects::node::Node;

/// Main struct for defining the profile of the developer (or person in the future)
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Dev {
    nodesDone: Vec<Node>,
    skills: Vec<String>, //currConfidence: HashMap<Node, i32> I need to study the ways I can do this
}

impl Dev {
    /// Constructor to create a new Dev profile
    #[allow(non_snake_case)]
    pub fn new(nodesDone: Vec<Node>, skills: Vec<String>) -> Self {
        Self { nodesDone, skills }
    }
}
