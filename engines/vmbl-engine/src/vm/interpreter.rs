use std::default;

use crate::{dictionary::{self, Bytecode}, vm::{constants::{self, interpret_double_byte, interpret_int_byte, interpret_string_byte}, interpreter, utils}};

pub struct Interpreter{
    pub constants: Vec<u8>,
    pub bytes: Vec<u8>,
    pub cursor: u32
}

#[allow(dead_code)]
impl Interpreter{
    pub fn new(bytes: Vec<u8>) -> Self{
        Interpreter { constants: Vec::new(), bytes, cursor: 0 }
    }

    fn read(amount: u8){
        
    }

    fn mount_buffered_constants(&mut self, usable_bytes: &Vec<u8>){
        let entries_to_read = u16::from_be_bytes([usable_bytes[1], usable_bytes[0]]); // the first element will always be the constants count on .ksc files

        for i in 0..entries_to_read{
            match usable_bytes[i] {
                11 => {
                    interpret_string_byte();
                },
                22 => {
                    interpret_int_byte();
                },
                33 => {
                    interpret_double_byte()
                },
                _ => println!("{}", i)
            }
        }
    }

    fn interpret_instruct(&mut self, byte: u8){
        println!("{:2x?}", self.constants);
    }

    pub fn interpret(&mut self){

        if !utils::is_bytecode_valid(&self.bytes){
            panic!("[ERROR] bytecode is invalid.\nhave you compiled it through VMBL?");
        }

        let useful_bytes: &[u8] = &self.bytes.clone()[4 .. self.bytes.len()]; // creates a slice without the headers
        
        if self.constants.is_empty(){ self.mount_buffered_constants(&useful_bytes.to_vec()); }

        for b in useful_bytes{
            self.interpret_instruct(*b);
        }
    }
}