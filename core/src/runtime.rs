use knowledge_engine::runtime_utils::Graph;

pub struct Runtime{
    pub graph: Graph
}

impl Runtime{
    pub fn new() -> Self{
        Self { graph: Graph::new() }
    }
}