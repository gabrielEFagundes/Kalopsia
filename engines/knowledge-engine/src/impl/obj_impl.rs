use lib::ss_engine::serializer::Serializer;

use crate::objects::{node::Node, obj::Obj};

impl Serializer for Obj{
    fn serialize(&self, buf: &mut Vec<shared::data_types::BYTE>) {
        for i in self.nodes_done(){
            Node::serialize(i, buf);
        }

        for i in self.skills(){
            Self::serialize_string(buf, i);
        }
    }

    fn deserialize(buf: &[shared::data_types::BYTE]) -> Self {
        todo!()
    }
}