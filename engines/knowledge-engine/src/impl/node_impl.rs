use std::collections::{BTreeMap, HashMap, HashSet};

use chrono::{DateTime, Local};
use lib::{id_engine::builder::Identifier, ss_engine::serializer::Serializer};
use shared::{data_types::BYTE, orderedf64::Orderedf64};

use crate::{
    enumerations::{Relationship, State},
    objects::{edge::Edge, node::Node},
};

impl Serializer for Node {
    fn serialize(&self, buf: &mut Vec<BYTE>) {
        Self::serialize_i32(buf, self.id.0);
        Self::serialize_string(buf, &self.name);
        Self::serialize_f64(buf, self.difficulty.0);
        Self::serialize_i32(buf, self.hours);

        Self::serialize_i32(buf, self.req_skills.len() as i32);
        for i in &self.req_skills {
            Self::serialize_string(buf, i);
        }

        Self::serialize_i32(buf, self.gain_skills.len() as i32);
        for i in &self.gain_skills {
            Self::serialize_string(buf, i);
        }

        Self::serialize_i32(buf, self.interest());
        Self::serialize_string(buf, &self.idea_added_at().to_string());
        Self::serialize_i32(buf, self.state() as i32);

        Self::serialize_i32(buf, self.connections().len() as i32);
        for i in self.connections() {
            for (k, v) in i {
                Self::serialize_i32(buf, k.0);
                Edge::serialize(v, buf);
            }
        }
    }

    fn deserialize(buf: &mut Vec<BYTE>, cursor: &mut usize) -> Self {
        let id = Identifier(Self::deserialize_i32(buf, cursor));
        let name = Self::deserialize_string(buf, cursor);
        let difficulty = Orderedf64(Self::deserialize_f64(buf, cursor));
        let hours = Self::deserialize_i32(buf, cursor);

        let mut arr_len = Self::deserialize_i32(buf, cursor);
        let mut req_skills: HashSet<String> = HashSet::new();
        for _ in 0..arr_len {
            req_skills.insert(Self::deserialize_string(buf, cursor));
        }

        arr_len = Self::deserialize_i32(buf, cursor);
        let mut gain_skills: HashSet<String> = HashSet::new();
        for _ in 0..arr_len {
            gain_skills.insert(Self::deserialize_string(buf, cursor));
        }

        let interest = Self::deserialize_i32(buf, cursor);
        let idea_added_at: DateTime<Local> = Self::deserialize_string(buf, cursor)
            .parse()
            .expect("[ERROR] can't resolve timestamp");
        let state = State::try_from(Self::deserialize_i32(buf, cursor)).unwrap();

        arr_len = Self::deserialize_i32(buf, cursor);
        let mut connections: Vec<HashMap<Identifier, Edge>> = Vec::new();
        for _ in 0..arr_len {
            let key = Identifier(Self::deserialize_i32(buf, cursor));

            let edge = Edge::new(
                Self::deserialize_i32(buf, cursor),
                Relationship::try_from(Self::deserialize_i32(buf, cursor)).unwrap(),
            );
            connections.push(HashMap::from([(key, edge)]));
        }

        Node::from(
            id,
            name,
            difficulty,
            hours,
            req_skills,
            gain_skills,
            interest,
            idea_added_at,
            state,
            connections,
        )
    }
}
