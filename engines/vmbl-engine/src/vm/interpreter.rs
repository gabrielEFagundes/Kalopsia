use core::panic;

use crate::{dictionary::{ByteTokenType, Bytecode, ValueType, as_bytecode, as_bytetokentype}, vm::{constants::{interpret_double_byte, interpret_int_byte, interpret_string_byte}, utils}};

pub struct Interpreter{
    pub constants: Vec<ValueType>,
    pub bytes: Vec<u8>,
    pub cursor: u32
}

#[allow(dead_code)]
impl Interpreter{
    pub fn new(bytes: Vec<u8>) -> Self{
        Interpreter { constants: Vec::new(), bytes, cursor: 0 }
    }

    pub fn read(&mut self) -> u8{
        self.cursor+=1;
        return self.bytes[self.cursor as usize];
    }

    fn read_amount(&mut self, amount: u32) -> u8{
        self.cursor += amount;
        return self.bytes[self.cursor as usize];
    }

    fn mount_buffered_constants(&mut self){
        let first_byte = self.bytes[self.cursor as usize];
        self.read();

        let entries_to_read = u16::from_be_bytes([self.bytes[self.cursor as usize], first_byte]);
        self.read();

        for _i in 0..=entries_to_read as usize{
            match as_bytetokentype(self.bytes[self.cursor as usize]) {
                ByteTokenType::STRING => {
                    self.read();
                    let str_length = self.bytes[self.cursor as usize];
                    let val = ValueType::Str(interpret_string_byte(self, str_length));
                    self.constants.push(val);
                    self.read();
                },

                ByteTokenType::INT => {
                    self.read();
                    let val = ValueType::Int(interpret_int_byte(self));
                    self.constants.push(val);
                },

                ByteTokenType::DOUBLE => {
                    self.read();
                    let val = ValueType::Double(interpret_double_byte(self));
                    self.constants.push(val);
                },

                _ => break
            }
        }
    }

    fn interpret_instruct(&mut self){
        let mut stack_indexes: Vec<ValueType> = Vec::new();

        while self.cursor < self.bytes.len() as u32{
            match as_bytecode(self.bytes[self.cursor as usize]) {
                Bytecode::PUSH => {
                    self.read();
                    stack_indexes.push(self.constants[self.bytes[self.cursor as usize] as usize].clone());

                    self.read();
                },

                Bytecode::DEFINE => {
                    println!("DEFINE");
                    break;
                },

                Bytecode::QUERY => {
                    println!("QUERY");
                    break;
                },

                Bytecode::NODE => {
                    stack_indexes.clear();
                    break;
                },

                Bytecode::OBJ => {
                    stack_indexes.clear();
                    break;
                },

                Bytecode::PATH => {
                    println!("PATH");
                    break;
                },

                Bytecode::MK_ARRAY => {
                    self.read();
                    
                    let count = self.bytes[self.cursor as usize];
                    let mut _disposable_arr: Vec<ValueType> = Vec::new();
                    for _i in 0..count{
                        _disposable_arr.push(stack_indexes.pop().unwrap().clone());
                    }
                    stack_indexes.push(ValueType::Vec(_disposable_arr));

                    self.read();
                },

                _ => {
                    println!("[DEBUG] Stopped at {}", self.bytes[self.cursor as usize]);
                    break;
                }
            }
        }
        println!("[DEBUG] non cleared pushed array: {:#?}", stack_indexes);
    }

    pub fn interpret(&mut self){

        if !utils::is_bytecode_valid(&self.bytes){
            panic!("[ERROR] bytecode is invalid.\nhave you compiled it through a compatible version of VMBL?");
        }

        self.read_amount(4);
        
        if self.constants.is_empty(){ self.mount_buffered_constants(); }
        //println!("{:#?}", self.constants);

        self.interpret_instruct();
    }
}