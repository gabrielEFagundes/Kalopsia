use std::collections::{BTreeMap, HashMap};
use knowledge_engine::enumerations::Relationship;
use knowledge_engine::objects::edge::Edge;
use knowledge_engine::objects::node::Node;

// reminder here to remove all the unecessary debug derives
fn main() {
    // println!("This is the knowledge engine, and this is also a temporary print.");
    // HOW THE FUCK IS THIS WORKING
    let mut node = Node::new(String::from("Idk"),
        7,
        20,
        vec!["Skill".to_string(), "Another skill".to_string()],
        vec!["Cool Skill".to_string(), "Idk another skill".to_string()],
    );

    let node2 = Node::new(String::from("Idk n° 2"),
          10,
          400,
          vec!["GREATTTTTT SKILLLLLLLLL".to_string(), "Another skill".to_string()],
          vec!["Shit Skill".to_string(), "No reason at all skill".to_string()]
    );

    let edge_between_node1and2 = Edge::new(8, Relationship::REQUIRED);

    let mut connection = BTreeMap::new();
    connection.insert(node2, edge_between_node1and2);
    node.add_conn(connection);

    println!("{:?}", node);
}
