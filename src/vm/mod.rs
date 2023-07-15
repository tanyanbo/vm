use crate::value::{is_number, number, Value};

const OP_HALT: u8 = 0x00;
const OP_CONST: u8 = 0x01;
const OP_ADD: u8 = 0x02;
const OP_SUB: u8 = 0x03;
const OP_MUL: u8 = 0x04;
const OP_DIV: u8 = 0x05;

enum MathOperation {
    ADD,
    SUB,
    MUL,
    DIV,
}

pub struct VM {
    constants: Vec<Value>,
    stack: Vec<Value>,
    bytecode: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            constants: vec![number(1.0), number(3.0)],
            stack: vec![],
            bytecode: vec![OP_CONST, 1, OP_CONST, 0, OP_ADD, OP_HALT],
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
                OP_ADD => {
                    let result = self.math_operation(MathOperation::ADD);
                    self.stack.push(number(result));
                }
                OP_SUB => {
                    let result = self.math_operation(MathOperation::SUB);
                    self.stack.push(number(result));
                }
                OP_MUL => {
                    let result = self.math_operation(MathOperation::MUL);
                    self.stack.push(number(result));
                }
                OP_DIV => {
                    let result = self.math_operation(MathOperation::DIV);
                    self.stack.push(number(result));
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
    }

    fn math_operation(&mut self, op: MathOperation) -> f64 {
        let num1 = self.stack.pop().unwrap();
        let num2 = self.stack.pop().unwrap();
        if is_number(&num1) && is_number(&num2) {
            match op {
                MathOperation::ADD => num1.number + num2.number,
                MathOperation::SUB => num1.number - num2.number,
                MathOperation::MUL => num1.number * num2.number,
                MathOperation::DIV => num1.number / num2.number,
            }
        } else {
            panic!("Operands must be numbers");
        }
    }
}
