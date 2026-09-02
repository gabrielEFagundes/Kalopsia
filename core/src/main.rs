use core::runtime::Runtime;
use std::collections::HashSet;

use knowledge_engine::{enumerations::Relationship, objects::{edge::Edge, node::Node, obj::Obj}};
use lib::{data_io, ss_engine::serializer::Serializer};
use shared::{debug, orderedf64::Orderedf64};

fn main() {
    let mut runtime = Runtime::new();

    let mut dev = Obj::new(HashSet::from(["Skill 1".to_string(), "Skill 2".to_string()]));

    let mut node = Node::new(
        String::from("NODE NUMBER 1"),
        Orderedf64(7.0),
        20,
        HashSet::from(["Skill".to_string(), "Another skill".to_string()]),
        HashSet::from(["Cool Skill".to_string(), "Idk another skill".to_string()]),
    );

    let mut node2 = Node::new(
        String::from("NODE NUMBER 2"),
        Orderedf64(10f64),
        400,
        HashSet::from([
            "GREATTTTTT SKILLLLLLLLL".to_string(),
            "Another skill".to_string(),
        ]),
        HashSet::from([
            "Shit Skill".to_string(),
            "No reason at all skill".to_string(),
        ]),
    );

    let edge_between_node1and2 = Edge::new(8, Relationship::REQUIRED, node.id);

    node.add_conn(&mut node2, edge_between_node1and2);
    dev.add_nodes(vec![&node, &node2]);

    // -- VMBL ENGINE TEST --
    //vmbl_engine::run(&mut runtime.graph);

    // -- DATA SERIALIZING & DESERIALIZING TEST --
    //serialize_deserialize_data(node);
}

#[allow(dead_code)]
fn serialize_deserialize_data(test_node: Node){
    let mut buffer: Vec<shared::data_types::BYTE> = Vec::new();
    test_node.serialize(&mut buffer);

    _ = data_io::appendf(&buffer, "./test-data.bin/graph.bin");

    let serialized_node = &mut data_io::readf("./test-data.bin/graph.bin");
    debug!("{:#?}", Node::deserialize(serialized_node, &mut 0));
}