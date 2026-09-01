use crate::{
    dictionary::{ByteTokenType, Bytecode, as_bytecode, as_bytetokentype},
    vm::{
        constants::{interpret_double_byte, interpret_int_byte, interpret_string_byte},
        interactions::{create_node, create_obj},
        utils,
    },
};
use core::panic;
use knowledge_engine::runtime_utils::Graph;
use shared::data_types::ValueType;

pub struct Interpreter {
    pub constants: Vec<ValueType>,
    pub bytes: Vec<u8>,
    pub cursor: u32,
}

#[allow(dead_code)]
impl Interpreter {
    pub fn new(bytes: Vec<u8>) -> Self {
        Interpreter {
            constants: Vec::new(),
            bytes,
            cursor: 0,
        }
    }

    pub fn read(&mut self) -> u8 {
        self.cursor += 1;
        self.bytes[self.cursor as usize]
    }

    fn read_amount(&mut self, amount: u32) -> u8 {
        self.cursor += amount;
        self.bytes[self.cursor as usize]
    }

    fn mount_buffered_constants(&mut self) {
        let first_byte = self.bytes[self.cursor as usize];
        self.read();

        let entries_to_read = u16::from_be_bytes([self.bytes[self.cursor as usize], first_byte]);
        self.read();

        for _i in 0..=entries_to_read as usize {
            match as_bytetokentype(self.bytes[self.cursor as usize]) {
                ByteTokenType::STRING => {
                    self.read();
                    let str_length = self.bytes[self.cursor as usize];
                    let val = ValueType::Str(interpret_string_byte(self, str_length));
                    self.constants.push(val);
                    self.read();
                }

                ByteTokenType::INT => {
                    self.read();
                    let val = ValueType::Int(interpret_int_byte(self));
                    self.constants.push(val);
                }

                ByteTokenType::DOUBLE => {
                    self.read();
                    let val = ValueType::Double(interpret_double_byte(self));
                    self.constants.push(val);
                }

                _ => break,
            }
        }
    }

    fn interpret_instruct(&mut self, graph: &mut Graph) {
        let mut stack_indexes: Vec<ValueType> = Vec::new();

        while self.cursor < self.bytes.len() as u32 {
            match as_bytecode(self.bytes[self.cursor as usize]) {
                Bytecode::PUSH => {
                    self.read();
                    stack_indexes
                        .push(self.constants[self.bytes[self.cursor as usize] as usize].clone());

                    self.read();
                }

                Bytecode::DEFINE => {
                    self.read();
                    let _current_byte_type = self.bytes[self.cursor as usize];

                    'node: {
                        if _current_byte_type == Bytecode::OBJ as u8 {
                            break 'node;
                        }

                        self.read();
                        create_node(&mut stack_indexes);

                        self.read();
                        break 'node;
                    };

                    'obj: {
                        if _current_byte_type == Bytecode::NODE as u8 {
                            break 'obj;
                        } // maybe an overhead, but works and is safe

                        self.read();
                        create_obj(&mut stack_indexes);

                        self.read();
                        break 'obj;
                    };
                }

                Bytecode::QUERY => {
                    self.read();
                    let _current_byte_type = self.bytes[self.cursor as usize];
                    /* -- just a kind reminder here, from Gabriel of 26/09/01 --
                    THE ID AND SS ENGINES WERE PAINFUL
                    genuinely, I hated writing them, both came out of nowhere because of 
                    problems I had while writing the interpreter of VMBL itself. 
                    
                    I'm glad it's over, but I'll never forget how terrible those were to write.*/

                    'node: {
                        if _current_byte_type != Bytecode::NODE as u8{ break 'node; }
                        break 'node;
                    }

                    'obj: {
                        if _current_byte_type != Bytecode::OBJ as u8{ break 'obj; }
                        break 'obj;
                    }

                    'path: {
                        if _current_byte_type != Bytecode::PATH as u8{ break 'path; }
                        break 'path;
                    }

                    'next: {
                        if _current_byte_type != Bytecode::NEXT as u8{ break 'next; }
                        break 'next;
                    }
                }

                Bytecode::MK_ARRAY => {
                    self.read();

                    let count = self.bytes[self.cursor as usize];
                    let mut _disposable_arr: Vec<ValueType> = Vec::new();
                    for _i in 0..count {
                        _disposable_arr.push(stack_indexes.pop().unwrap().clone());
                    }
                    stack_indexes.push(ValueType::DVec(_disposable_arr));

                    self.read();
                }

                _ => {
                    println!("[DEBUG] Stopped at {}", self.bytes[self.cursor as usize]);
                    break;
                }
            }
        }
        println!("[DEBUG] cleared pushed array: {:#?}", stack_indexes);
    }

    pub fn interpret(&mut self, graph: &mut Graph) {
        if !utils::is_bytecode_valid(&self.bytes) {
            panic!(
                "[ERROR] bytecode is invalid.\nhave you compiled it through a compatible version of VMBL?"
            );
        }

        self.read_amount(4);

        if self.constants.is_empty() {
            self.mount_buffered_constants();
        }
        //println!("{:#?}", self.constants);

        self.interpret_instruct(graph);
    }
}
