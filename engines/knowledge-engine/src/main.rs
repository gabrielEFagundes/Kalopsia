use knowledge_engine::objects::node::Node;

// reminder here to remove all the unecessary debug derives
fn main() {
    // println!("This is the knowledge engine, and this is also a temporary print.");
    let node = Node::new(String::from("Idk"), 
        7, 
        20, 
        vec!["Skill".to_string(), "Another skill".to_string()], 
        vec!["Cool Skill".to_string(), "Idk another skill".to_string()]
    );

    println!("{:?}", node);
}
