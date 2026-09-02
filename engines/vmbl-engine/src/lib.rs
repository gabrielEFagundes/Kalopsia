mod dictionary;
pub mod vm;

use std::fs;

use crate::vm::interpreter::Interpreter;
use knowledge_engine::runtime_utils::Graph;

const TARGET_FILE: &str = "./out/target.ksc";

pub fn run(graph: &mut Graph) {
    let content = fs::read(TARGET_FILE);
    if content.is_err() {
        panic!(
            "[ERROR] an error occurred when trying to read target file {}: {}",
            TARGET_FILE,
            content.unwrap_err()
        );
    }

    let mut interpreter = Interpreter::new(content.unwrap());

    interpreter.interpret(graph);
}
