use crate::enumerations::Relationship;

/// Main struct used for the connections between the nodes on the graph
#[derive(Debug, Clone, Copy)]
pub struct Edge{
    weight: f64,
    relationship: Relationship
}

impl Edge{
    /// Constructor with the required fields to create an Edge
    pub fn new(weight: f64, relationship: Relationship) -> Self {
        Self { weight, relationship }
    }
}