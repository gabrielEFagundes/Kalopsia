use lib::{ss_engine::serializer::Serializer};
use shared::data_types::BYTE;

use crate::objects::{edge::Edge, node::Node};

impl Serializer for Node{
    fn serialize(&self, buf: &mut Vec<BYTE>){
        Self::serialize_i32(buf, self.id.0);
        Self::serialize_string(buf, &self.name);
        Self::serialize_f64(buf, self.difficulty.0);
        Self::serialize_i32(buf, self.hours);

        for i in &self.reqSkills{
            Self::serialize_string(buf, i);
        }

        for i in &self.gainSkills{
            Self::serialize_string(buf, i);
        }

        Self::serialize_i32(buf, self.interest());
        Self::serialize_string(buf, &self.idea_added_at().to_string());
        buf.push(self.state() as u8);

        for i in self.connections(){
            for (k, v) in i{
                Self::serialize_i32(buf, k.0);
                Edge::serialize(v, buf);
            }
        }
    }

    fn deserialize(buf: &[u8]) -> Self {
        todo!()
    }
}