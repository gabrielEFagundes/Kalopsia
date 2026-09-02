use knowledge_engine::runtime_utils::Graph;

pub struct Runtime {
    pub graph: Graph,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }
}
