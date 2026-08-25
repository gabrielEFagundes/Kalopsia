use std::fs;

use vmbl_engine::vm::interpreter::Interpreter;

const TARGET_FILE: &str = "./out/target.ksc";

fn main() {
    let content = fs::read(TARGET_FILE);
    if content.is_err(){
        panic!("[ERROR] an error occurred when trying to read target file {}: {}", TARGET_FILE, content.unwrap_err().to_string());
    }

    let mut interpreter = Interpreter::new(content.unwrap());

    interpreter.interpret();
}
