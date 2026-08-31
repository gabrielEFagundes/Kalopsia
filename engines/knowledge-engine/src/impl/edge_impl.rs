use lib::ss_engine::serializer::Serializer;
use shared::data_types::BYTE;

use crate::{enumerations::Relationship, objects::edge::Edge};

impl Serializer for Edge {
    fn serialize(&self, buf: &mut Vec<BYTE>) {
        Self::serialize_i32(buf, self.weight());
        Self::serialize_i32(buf, self.relationship() as i32);
    }

    fn deserialize(buf: &mut Vec<BYTE>, cursor: &mut usize) -> Self {
        let weight = Self::deserialize_i32(buf, cursor);
        let relationship = Relationship::try_from(Self::deserialize_i32(buf, cursor)).unwrap();

        Edge::new(weight, relationship)
    }
}
