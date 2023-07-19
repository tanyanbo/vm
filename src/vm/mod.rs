use crate::{
    compiler::CompileResult,
    value::{boolean, number, string, Value},
};

pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;
pub const OP_SUB: u8 = 0x03;
pub const OP_MUL: u8 = 0x04;
pub const OP_DIV: u8 = 0x05;
pub const OP_GT: u8 = 0x06;
pub const OP_GTE: u8 = 0x07;
pub const OP_LT: u8 = 0x08;
pub const OP_LTE: u8 = 0x09;
pub const OP_EQ: u8 = 0x0a;
pub const OP_JUMP_IF_FALSE: u8 = 0x0b;
pub const OP_JUMP: u8 = 0x0c;

enum MathOperation {
    ADD,
    SUB,
    MUL,
    DIV,
}

enum ComparisonOperation {
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Equal,
}

pub struct VM {
    constants: Vec<Value>,
    stack: Vec<Value>,
    bytecode: Vec<u8>,
}

impl VM {
    pub fn new(compile_result: CompileResult) -> VM {
        VM {
            constants: compile_result.constants,
            stack: vec![],
            bytecode: compile_result.bytecode,
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
                OP_GT => {
                    let result = self.comparison_operation(ComparisonOperation::Greater);
                    self.stack.push(result)
                }
                OP_GTE => {
                    let result = self.comparison_operation(ComparisonOperation::GreaterEqual);
                    self.stack.push(result)
                }
                OP_LT => {
                    let result = self.comparison_operation(ComparisonOperation::Lesser);
                    self.stack.push(result)
                }
                OP_LTE => {
                    let result = self.comparison_operation(ComparisonOperation::LesserEqual);
                    self.stack.push(result)
                }
                OP_EQ => {
                    let result = self.comparison_operation(ComparisonOperation::Equal);
                    self.stack.push(result)
                }
                OP_JUMP_IF_FALSE => {
                    let result = self.stack.pop().unwrap();
                    if let Value::Boolean { val } = result {
                        if !val {
                            ip = (self.bytecode[ip]) as usize;
                        } else {
                            ip += 1;
                        }
                    } else {
                        panic!("Invalid condition expression");
                    }
                }
                OP_JUMP => {
                    ip = (self.bytecode[ip]) as usize;
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
    }

    fn comparison_operation(&mut self, op: ComparisonOperation) -> Value {
        let val2 = self.stack.pop().unwrap();
        let val1 = self.stack.pop().unwrap();

        if let (Value::Boolean { val: bool1 }, Value::Boolean { val: bool2 }) = (&val1, &val2) {
            VM::comparision_fn(op, bool1, bool2)
        } else if let (Value::Number { val: num1 }, Value::Number { val: num2 }) = (&val1, &val2) {
            VM::comparision_fn(op, num1, num2)
        } else if let (Value::String { val: str1 }, Value::String { val: str2 }) = (&val1, &val2) {
            VM::comparision_fn(op, str1, str2)
        } else {
            panic!("Invalid operands");
        }
    }

    fn comparision_fn<T>(op: ComparisonOperation, val1: T, val2: T) -> Value
    where
        T: PartialOrd + PartialEq,
    {
        match op {
            ComparisonOperation::Greater => boolean(val1 > val2),
            ComparisonOperation::GreaterEqual => boolean(val1 >= val2),
            ComparisonOperation::Lesser => boolean(val1 < val2),
            ComparisonOperation::LesserEqual => boolean(val1 <= val2),
            ComparisonOperation::Equal => boolean(val1 == val2),
        }
    }

    fn math_operation(&mut self, op: MathOperation) -> Value {
        let val2 = self.stack.pop().unwrap();
        let val1 = self.stack.pop().unwrap();

        if let (Value::Number { val: num1 }, Value::Number { val: num2 }) = (&val1, &val2) {
            match op {
                MathOperation::ADD => number(num1 + num2),
                MathOperation::SUB => number(num1 - num2),
                MathOperation::MUL => number(num1 * num2),
                MathOperation::DIV => number(num1 / num2),
            }
        } else if let (Value::String { val: str1 }, Value::String { val: str2 }) = (&val1, &val2) {
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
