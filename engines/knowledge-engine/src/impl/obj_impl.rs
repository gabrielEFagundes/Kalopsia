use std::{collections::HashSet};

use lib::ss_engine::serializer::Serializer;
use shared::data_types::BYTE;

use crate::objects::{node::Node, obj::Obj};

impl Serializer for Obj {
    fn serialize(&self, buf: &mut Vec<BYTE>) {
        Self::serialize_i32(buf, self.nodes_done().len() as i32);
        for i in self.nodes_done() {
            Node::serialize(i, buf);
        }

        Self::serialize_i32(buf, self.skills().len() as i32);
        for i in self.skills() {
            Self::serialize_string(buf, i);
        }
    }

    fn deserialize(buf: &mut Vec<BYTE>, cursor: &mut usize) -> Self {
        let mut arr_len = Self::deserialize_i32(buf, cursor);
        let mut nodes_done: Vec<Node> = Vec::new();
        for _ in 0..arr_len {
            nodes_done.push(Node::deserialize(buf, cursor));
        }

        arr_len = Self::deserialize_i32(buf, cursor);
        let mut skills: HashSet<String> = HashSet::new();
        for _ in 0..arr_len {
            skills.insert(Self::deserialize_string(buf, cursor));
        }

        Obj::from(nodes_done, skills)
    }
}
