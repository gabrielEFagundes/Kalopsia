use lib::ss_engine::serializer::Serializer;
use shared::data_types::BYTE;

use crate::objects::edge::Edge;

impl Serializer for Edge{
    fn serialize(&self, buf: &mut Vec<BYTE>){
        Self::serialize_i32(buf, self.weight());
        buf.push(self.relationship() as u8);
    }

    fn deserialize(buf: &[BYTE]) -> Self {
        todo!()
    }
}