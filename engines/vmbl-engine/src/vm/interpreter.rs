use core::panic;

use crate::{dictionary::{ByteTokenType, ValueType, as_bytetokentype}, vm::{constants::{interpret_double_byte, interpret_int_byte, interpret_string_byte}, utils}};

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

    fn interpret_instruct(&mut self, byte: u8){
        println!("{:2x?}", self.constants);
    }

    pub fn interpret(&mut self){

        if !utils::is_bytecode_valid(&self.bytes){
            panic!("[ERROR] bytecode is invalid.\nhave you compiled it through VMBL?");
        }

        self.read_amount(4);
        
        if self.constants.is_empty(){ self.mount_buffered_constants(); }
        println!("{:#?}", self.constants);

        // for b in self.bytes{
        //     self.interpret_instruct(*b);
        // }
    }
}