use lib::id_engine::builder::Identifier;

use crate::enumerations::Relationship;

/// Main struct used for the connections between the nodes on the graph
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Edge {
    weight: i32,
    relationship: Relationship,
    direction: Identifier,
}

impl Edge {
    /// Constructor with the required fields to create an Edge
    pub fn new(weight: i32, relationship: Relationship, direction: Identifier) -> Self {
        Self {
            weight,
            relationship,
            direction,
        }
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }

    pub fn relationship(&self) -> Relationship {
        self.relationship
    }

    pub fn direction(&self) -> &Identifier {
        &self.direction
    }
}
