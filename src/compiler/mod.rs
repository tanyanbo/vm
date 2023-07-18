use crate::{
    parser::{AstNode, BinaryExpressionType, LiteralType},
    value::Value,
    vm::{OP_ADD, OP_CONST, OP_DIV, OP_MUL, OP_SUB},
};

#[derive(Debug)]
pub struct CompileResult {
    pub bytecode: Vec<u8>,
    pub constants: Vec<Value>,
}

pub struct Compiler {
    pub result: CompileResult,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            result: CompileResult {
                bytecode: vec![],
                constants: vec![],
            },
        }
    }

    pub fn compile(&mut self, ast: AstNode) {
        if let AstNode::Program { children } = ast {
            for child in children {
                self.compile_binary_expression(child);
            }
        } else {
            panic!("Invalid AST");
        }
    }

    fn compile_binary_expression(&mut self, node: AstNode) {
        if let AstNode::BinaryExpression {
            r#type: binary_expression_type,
            left,
            right,
        } = node
        {
            self.compile_binary_expression_part(*left);
            self.compile_binary_expression_part(*right);

            match binary_expression_type {
                BinaryExpressionType::Add => {
                    self.emit(OP_ADD);
                }
                BinaryExpressionType::Sub => {
                    self.emit(OP_SUB);
                }
                BinaryExpressionType::Mul => {
                    self.emit(OP_MUL);
                }
                BinaryExpressionType::Div => {
                    self.emit(OP_DIV);
                }
            }
        }
    }

    fn compile_binary_expression_part(&mut self, node: AstNode) {
        if let AstNode::Literal {
            r#type: literal_type,
            value,
        } = node
        {
            match literal_type {
                LiteralType::Number => {
                    self.compile_constant(Value::Number {
                        num: value.parse::<f64>().unwrap(),
                    });
                }
                LiteralType::String => {
                    self.compile_constant(Value::String { str: value });
                }
            }
        } else {
            self.compile_binary_expression(node);
        }
    }

    fn compile_constant(&mut self, value: Value) {
        if self.result.constants.len() > 254 {
            panic!("Too many constants");
        }

        self.emit(OP_CONST);

        for i in 0..self.result.constants.len() {
            match &self.result.constants[i] {
                Value::Number { num: constant_num } => {
                    if let Value::Number { num: value_num } = &value {
                        if constant_num == value_num {
                            self.emit(i as u8);
                            return;
                        }
                    }
                }
                Value::String { str: constant_str } => {
                    if let Value::String { str: value_str } = &value {
                        if constant_str == value_str {
                            self.emit(i as u8);
                            return;
                        }
                    }
                }
            }
        }

        self.result.constants.push(value);
        self.emit((self.result.constants.len() - 1) as u8);
    }

    fn emit(&mut self, byte: u8) {
        self.result.bytecode.push(byte);
    }
}
