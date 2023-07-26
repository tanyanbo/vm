use crate::value::{boolean, number, string, Value};

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
pub const OP_SET_VAR: u8 = 0x0d;
pub const OP_GET_VAR: u8 = 0x0e;
pub const OP_POP: u8 = 0x0f;
pub const OP_SCOPE_EXIT: u8 = 0x10;
pub const OP_CALL: u8 = 0x11;
pub const OP_RETURN: u8 = 0x12;

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

const STACK_SIZE: usize = 512;

pub struct VM {
    stack: [Option<Value>; STACK_SIZE],
    sp: usize,
    bp: usize,
}

const fn init_value() -> Option<Value> {
    None
}

const VALUE_INITIAL: Option<Value> = init_value();

impl VM {
    pub fn new() -> VM {
        let sp = 0;

        VM {
            stack: [VALUE_INITIAL; STACK_SIZE],
            sp,
            bp: sp,
        }
    }

    pub fn exec(&mut self, constants: Vec<Value>, bytecode: Vec<u8>) -> Value {
        let mut ip: usize = 0;

        loop {
            let instruction = bytecode[ip];
            ip += 1;

            match instruction {
                OP_HALT => {
                    return self.stack_pop();
                }
                OP_CONST => {
                    let constant = constants[bytecode[ip] as usize].clone();
                    ip += 1;
                    self.stack_push(constant);
                }
                OP_ADD => {
                    let result = self.math_operation(MathOperation::ADD);
                    self.stack_push(result);
                }
                OP_SUB => {
                    let result = self.math_operation(MathOperation::SUB);
                    self.stack_push(result);
                }
                OP_MUL => {
                    let result = self.math_operation(MathOperation::MUL);
                    self.stack_push(result);
                }
                OP_DIV => {
                    let result = self.math_operation(MathOperation::DIV);
                    self.stack_push(result);
                }
                OP_GT => {
                    let result = self.comparison_operation(ComparisonOperation::Greater);
                    self.stack_push(result);
                }
                OP_GTE => {
                    let result = self.comparison_operation(ComparisonOperation::GreaterEqual);
                    self.stack_push(result);
                }
                OP_LT => {
                    let result = self.comparison_operation(ComparisonOperation::Lesser);
                    self.stack_push(result);
                }
                OP_LTE => {
                    let result = self.comparison_operation(ComparisonOperation::LesserEqual);
                    self.stack_push(result);
                }
                OP_EQ => {
                    let result = self.comparison_operation(ComparisonOperation::Equal);
                    self.stack_push(result);
                }
                OP_JUMP_IF_FALSE => {
                    let result = self.stack_pop();
                    if let Value::Boolean { val } = result {
                        if !val {
                            ip = (bytecode[ip]) as usize;
                        } else {
                            ip += 1;
                        }
                    } else {
                        panic!("Invalid condition expression");
                    }
                }
                OP_JUMP => {
                    ip = (bytecode[ip]) as usize;
                }
                OP_GET_VAR => {
                    let position = bytecode[ip];
                    ip += 1;

                    let value = self.peek(position as usize);

                    self.stack_push(value.clone());
                    if let Value::Function { .. } = value {
                        self.bp = self.sp - 1;
                    }
                }
                OP_SET_VAR => {
                    let position = bytecode[ip];
                    ip += 1;

                    let value = self.peek(self.sp - 1);
                    self.stack_set(position as usize, value.clone());
                }
                OP_POP => {
                    self.stack_pop();
                }
                OP_SCOPE_EXIT => {
                    let result = self.stack_pop();

                    let number_of_vars_to_pop = bytecode[ip];
                    ip += 1;

                    // There is an edge case where the block has only one variable
                    // declaration expression. In that case,
                    // the stack pointer will be equal to the base pointer and we
                    // don't want to move the stack pointer.
                    if self.sp != self.bp || number_of_vars_to_pop != 1 {
                        self.sp -= number_of_vars_to_pop as usize;
                    }

                    self.stack_push(result);
                }
                OP_CALL => {
                    let function = self.peek(0);
                    if let Value::Function {
                        bytecode,
                        constants,
                        ..
                    } = function
                    {
                        self.exec(constants, bytecode);
                    }
                }
                OP_RETURN => {
                    return self.stack_pop();
                }
                _ => panic!("Unknown instruction {}", instruction),
            }
        }
    }

    fn comparison_operation(&mut self, op: ComparisonOperation) -> Value {
        let val2 = self.stack_pop();
        let val1 = self.stack_pop();

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
        let val2 = self.stack_pop();
        let val1 = self.stack_pop();

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

    fn stack_pop(&mut self) -> Value {
        self.sp -= 1;

        if let Some(value) = &self.stack[self.sp] {
            value.clone()
        } else {
            panic!("Stack underflow");
        }
    }

    fn stack_push(&mut self, value: Value) {
        self.stack[self.sp] = Some(value);
        self.sp += 1;
    }

    fn peek(&mut self, offset: usize) -> Value {
        if let Some(value) = &self.stack[self.bp + offset] {
            value.clone()
        } else {
            panic!("Invalid stack offset");
        }
    }

    fn stack_set(&mut self, offset: usize, value: Value) {
        self.stack[offset] = Some(value);
    }
}
