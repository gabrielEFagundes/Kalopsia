use crate::objects::{edge::Edge, node::Node, obj::Obj};

pub struct Graph{
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: Vec<Edge>,
    pub(crate) objects: Vec<Obj>,
}

impl Graph{
    pub fn new() -> Self{
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

    pub fn add_node(&mut self, node: Node){
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge){
        self.edges.push(edge);
    }

    pub fn add_object(&mut self, obj: Obj){
        self.objects.push(obj);
    }
}