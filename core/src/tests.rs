use knowledge_engine::objects::node::Node;
use lib::{data_io, ss_engine::serializer::Serializer};
use shared::debug;

use crate::runtime::Runtime;

pub fn main_test(runtime: &mut Runtime){

    // few dummies to test the formulas
    for _ in 0..8{
        runtime.graph.add_node(Node::dummy());
    }

    lib::pg_engine::formulas::Formulas::Circular.convert((0.0, 100.0), (0.0, 100.0), runtime.graph.nodes().len());

    // -- VMBL ENGINE TEST --
    //vmbl_engine::run(&mut runtime.graph);

    // -- DATA SERIALIZING & DESERIALIZING TEST --
    //sddata_test(node);
}

#[allow(dead_code)]
fn sddata_test(test_node: Node) {
    let mut buffer: Vec<shared::data_types::BYTE> = Vec::new();
    test_node.serialize(&mut buffer);

    _ = data_io::appendf(&buffer, "./test-data.bin/graph.bin");

    let serialized_node = &mut data_io::readf("./test-data.bin/graph.bin");
    debug!("{:#?}", Node::deserialize(serialized_node, &mut 0));
}