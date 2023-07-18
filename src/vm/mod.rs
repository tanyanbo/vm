use crate::value::{number, string, Value};

pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;
pub const OP_SUB: u8 = 0x03;
pub const OP_MUL: u8 = 0x04;
pub const OP_DIV: u8 = 0x05;

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
            constants: vec![string("Hello ".to_string()), string("World!".to_string())],
            stack: vec![],
            bytecode: vec![OP_CONST, 0, OP_CONST, 1, OP_ADD, OP_HALT],
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
                    self.stack.push(result);
                }
                OP_SUB => {
                    let result = self.math_operation(MathOperation::SUB);
                    self.stack.push(result);
                }
                OP_MUL => {
                    let result = self.math_operation(MathOperation::MUL);
                    self.stack.push(result);
                }
                OP_DIV => {
                    let result = self.math_operation(MathOperation::DIV);
                    self.stack.push(result);
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
    }

    fn math_operation(&mut self, op: MathOperation) -> Value {
        let val2 = self.stack.pop().unwrap();
        let val1 = self.stack.pop().unwrap();

        if let (Value::Number { num: num1 }, Value::Number { num: num2 }) = (&val1, &val2) {
            match op {
                MathOperation::ADD => number(num1 + num2),
                MathOperation::SUB => number(num1 - num2),
                MathOperation::MUL => number(num1 * num2),
                MathOperation::DIV => number(num1 / num2),
            }
        } else if let (Value::String { str: str1 }, Value::String { str: str2 }) = (&val1, &val2) {
            match op {
                MathOperation::ADD => {
                    let mut result = str1.clone();
                    result.push_str(&str2);
                    string(result)
                }
                _ => panic!("Invalid operands"),
            }
        } else {
            panic!("Invalid operands");
        }
    }
}
