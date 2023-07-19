use crate::{
    parser::{AstNode, BinaryExpressionType, LiteralType},
    value::Value,
    vm::*,
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
            for expression in children {
                self.expression(expression);
            }
            self.emit(OP_HALT);
        } else {
            panic!("Invalid AST");
        }
    }

    fn expression(&mut self, expression: AstNode) {
        match expression {
            AstNode::BinaryExpression { .. } => {
                self.binary_expression(expression);
            }
            AstNode::IfExpression { .. } => {
                self.if_expression(expression);
            }
            AstNode::VariableDeclaration { .. } => {
                self.variable_declaration(expression);
            }
            AstNode::Literal {
                r#type: literal_type,
                value,
            } => {
                match literal_type {
                    LiteralType::Number => {
                        self.constant(Value::Number {
                            val: value.parse::<f64>().unwrap(),
                        });
                    }
                    LiteralType::String => {
                        self.constant(Value::String { val: value });
                    }
                    LiteralType::Boolean => {
                        self.constant(Value::Boolean {
                            val: value.parse::<bool>().unwrap(),
                        });
                    }
                };
            }
            _ => {
                panic!("Invalid AST");
            }
        }
    }

    fn variable_declaration(&mut self, node: AstNode) {
        todo!("Variable declaration");
    }

    fn if_expression(&mut self, node: AstNode) {
        if let AstNode::IfExpression {
            condition,
            consequent,
            alternate,
        } = node
        {
            self.expression(*condition);

            self.emit(OP_JUMP_IF_FALSE);
            self.emit(0);
            let jump_if_false_address = self.result.bytecode.len() - 1;

            self.expression(*consequent);

            self.emit(OP_JUMP);
            self.emit(0);
            let jump_address = self.result.bytecode.len() - 1;

            self.result.bytecode[jump_if_false_address] = self.result.bytecode.len() as u8;

            self.expression(*alternate);

            self.result.bytecode[jump_address] = self.result.bytecode.len() as u8;
        }
    }

    fn binary_expression(&mut self, node: AstNode) {
        if let AstNode::BinaryExpression {
            r#type: binary_expression_type,
            left,
            right,
        } = node
        {
            self.expression(*left);
            self.expression(*right);

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
                BinaryExpressionType::Greater => {
                    self.emit(OP_GT);
                }
                BinaryExpressionType::GreaterEqual => {
                    self.emit(OP_GTE);
                }
                BinaryExpressionType::Lesser => {
                    self.emit(OP_LT);
                }
                BinaryExpressionType::LesserEqual => {
                    self.emit(OP_LTE);
                }
                BinaryExpressionType::Equal => {
                    self.emit(OP_EQ);
                }
            }
        }
    }

    fn constant(&mut self, value: Value) {
        if self.result.constants.len() > 254 {
            panic!("Too many constants");
        }

        self.emit(OP_CONST);

        for i in 0..self.result.constants.len() {
            match &self.result.constants[i] {
                Value::Number { val: constant_num } => {
                    if let Value::Number { val: value_num } = &value {
                        if constant_num == value_num {
                            self.emit(i as u8);
                            return;
                        }
                    }
                }
                Value::String { val: constant_str } => {
                    if let Value::String { val: value_str } = &value {
                        if constant_str == value_str {
                            self.emit(i as u8);
                            return;
                        }
                    }
                }
                Value::Boolean { val: constant_val } => {
                    if let Value::Boolean { val: value_val } = &value {
                        if constant_val == value_val {
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
