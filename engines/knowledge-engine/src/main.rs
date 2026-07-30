use knowledge_engine::data_io;
use knowledge_engine::enumerations::Relationship;
use knowledge_engine::objects::dev::Dev;
use knowledge_engine::objects::edge::Edge;
use knowledge_engine::objects::node::Node;

// reminder here to remove all the unecessary debug derives
fn main() {
    // println!("This is the knowledge engine, and this is also a temporary print.");
    // HOW THE FUCK IS THIS WORKING
    let mut dev = Dev::new(vec!["Skill 1".to_string(), "Skill 2".to_string()]);

    let mut node = Node::new(
        String::from("NODE NUMBER 1"),
        7,
        20,
        vec!["Skill".to_string(), "Another skill".to_string()],
        vec!["Cool Skill".to_string(), "Idk another skill".to_string()],
    );

    let mut node2 = Node::new(
        String::from("NODE NUMBER 2"),
        10,
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

    //data_io::writef(node, "graph.hjson");
    data_io::appendf(node2, "graph.hjson");
}
