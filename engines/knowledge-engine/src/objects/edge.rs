use crate::enumerations::Relationship;

/// Main struct used for the connections between the nodes on the graph
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge {
    weight: i32,
    relationship: Relationship,
}

impl Edge {
    /// Constructor with the required fields to create an Edge
    pub fn new(weight: i32, relationship: Relationship) -> Self {
        Self {
            weight,
            relationship,
        }
    }

    pub fn weight(&self) -> i32{ self.weight }

    pub fn relationship(&self) -> Relationship{ self.relationship }
}
