use crate::value::{number, Value};

const OP_HALT: u8 = 0x00;
const OP_CONST: u8 = 0x01;
const OP_ADD: u8 = 0x02;
const OP_SUB: u8 = 0x03;
const OP_MUL: u8 = 0x04;
const OP_DIV: u8 = 0x05;

pub struct VM {
    constants: Vec<Value>,
    stack: Vec<Value>,
    bytecode: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            constants: vec![number(1.0)],
            stack: vec![],
            bytecode: vec![OP_CONST, 0, OP_HALT],
        }
    }

    pub fn exec(&mut self) -> Value {
        let mut ip: usize = 0;

        loop {
            let instruction = self.bytecode[ip];
            ip += 1;

            match instruction {
                OP_HALT => {
                    return self.stack.pop().unwrap_or_else(|| number(0.0));
                }
                OP_CONST => {
                    let constant = self.constants[self.bytecode[ip] as usize].clone();
                    ip += 1;
                    self.stack.push(constant);
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
    }
}
