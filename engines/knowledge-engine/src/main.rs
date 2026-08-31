use knowledge_engine::enumerations::Relationship;
use knowledge_engine::objects::obj::Obj;
use knowledge_engine::objects::edge::Edge;
use knowledge_engine::objects::node::Node;
use lib::data_io;
use shared::debug;
use lib::ss_engine::serializer::Serializer;
use shared::orderedf64::Orderedf64;

/// ## TEST FUNCTION
/// For the Knowledge Engine, this is not used by the main Kalopsia software.
/// 
/// Simply ignore this file if you're not here to test this specific module.
fn main() {
    let mut dev = Obj::new(vec!["Skill 1".to_string(), "Skill 2".to_string()]);

    let mut node = Node::new(
        String::from("NODE NUMBER 1"),
        Orderedf64(7.0),
        20,
        vec!["Skill".to_string(), "Another skill".to_string()],
        vec!["Cool Skill".to_string(), "Idk another skill".to_string()],
    );

    let mut node2 = Node::new(
        String::from("NODE NUMBER 2"),
        Orderedf64(10f64),
        400,
        vec![
            "GREATTTTTT SKILLLLLLLLL".to_string(),
            "Another skill".to_string(),
        ],
        vec![
            "Shit Skill".to_string(),
            "No reason at all skill".to_string(),
        ],
    );

    let edge_between_node1and2 = Edge::new(8, Relationship::REQUIRED);

    node.add_conn(&mut node2, edge_between_node1and2);
    dev.add_nodes(vec![&node, &node2]);

    let mut buffer: Vec<shared::data_types::BYTE> = Vec::new();
    node.serialize(&mut buffer);

    _ = data_io::appendf(&buffer, "./test-data.bin/graph.bin");

    let serialized_node = &mut data_io::readf("./test-data.bin/graph.bin");
    debug!("{:#?}", Node::deserialize(serialized_node, &mut 0));

}