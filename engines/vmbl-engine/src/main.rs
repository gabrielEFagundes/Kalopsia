use vmbl_engine::vm::interpreter::interpret;

use std::fs;

const TARGET_FILE: &str = "./out/target.ksc";

fn main() {
    let content = fs::read(TARGET_FILE);
    if content.is_err(){
        panic!("[ERROR] an error occurred when trying to read target file {}: {}", TARGET_FILE, content.unwrap_err().to_string());
    }

    interpret(content.unwrap());
}
