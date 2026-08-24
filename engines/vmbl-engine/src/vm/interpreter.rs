use crate::{dictionary::{self, Bytecode}, vm::utils};

const CONSTANTS: Vec<u8> = Vec::new();

fn mount_buffered_constants(constant: u8){
    println!("This shall add constants to the CONSTANTS array");
}

fn interpret_instruct(byte: u8){
    println!("It is a valid bytecode, so any errors now are VMBL's compiler fault.");
}

pub fn interpret(bytes: Vec<u8>){
    if !utils::is_bytecode_valid(&bytes){
        panic!("[ERROR] bytecode is invalid.\nhave you compiled it through VMBL?");
    }

    let useful_bytes = &bytes[4 .. bytes.len()]; // creates a slice without the headers
    
    for i in 0..useful_bytes[0]{
        mount_buffered_constants(i);
    }

    for b in useful_bytes{
        interpret_instruct(*b);
    }
}