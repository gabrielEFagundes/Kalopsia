use crate::objects::{edge::Edge, node::Node, obj::Obj};

pub struct Graph{
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: Vec<Edge>,
    pub(crate) objects: Vec<Obj>,
}

impl Graph{
    fn new() -> Self{
        Self { nodes: Vec::new(), edges: Vec::new(), objects: Vec::new() }
    }

    pub fn nodes(&self) -> &Vec<Node>{
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Edge>{
        &self.edges
    }

    pub fn objects(&self) -> &Vec<Obj>{
        &self.objects
    }
}

/// ## Warning
/// This WILL move places, since it's the entire 
/// Kalopsia's runtime struct.
/// 
/// It's inside `runtime_utils` merely because 
/// the knowledge engine is the only available at 
/// the moment.
pub struct Runtime{
    pub graph: Graph
}

impl Runtime{
    pub fn new() -> Self{
        Self { graph: Graph::new() }
    }
}