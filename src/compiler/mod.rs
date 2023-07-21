use crate::{
    parser::{AstNode, BinaryExpressionType, LiteralType},
    value::Value,
    vm::*,
};

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub scope_level: u8,
}

#[derive(Debug)]
pub struct CompileResult {
    pub bytecode: Vec<u8>,
    pub constants: Vec<Value>,
    pub vars: Vec<Var>,
}

pub struct Compiler {
    pub result: CompileResult,
    scope_level: u8,
    is_debug: bool,
}

impl Compiler {
    pub fn new(is_debug: bool) -> Compiler {
        Compiler {
            result: CompileResult {
                bytecode: vec![],
                constants: vec![],
                vars: vec![],
            },
            scope_level: 0,
            is_debug,
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
            AstNode::SetVariable { .. } => {
                self.set_variable(expression);
            }
            AstNode::Identifier { .. } => {
                self.identifier(expression);
            }
            AstNode::Block { .. } => {
                self.block_expression(expression);
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

    fn block_expression(&mut self, node: AstNode) {
        if let AstNode::Block { children } = node {
            self.scope_enter();

            let children_len = children.len();

            for (index, child) in children.into_iter().enumerate() {
                let should_pop = match child {
                    AstNode::VariableDeclaration { .. } => false,
                    _ => true,
                } && index != children_len - 1;

                self.expression(child);

                if should_pop {
                    self.emit(OP_POP);
                }
            }

            self.scope_exit();
        }
    }

    fn identifier(&mut self, node: AstNode) {
        if let AstNode::Identifier { name } = node {
            if self.result.vars.len() == 0 {
                panic!("Variable: {} not found", name);
            }

            self.emit(OP_GET_VAR);

            for i in (0..self.result.vars.len()).rev() {
                if self.result.vars[i].name == name {
                    self.emit(i as u8);
                    return;
                }
            }

            panic!("Variable: {} not found", name);
        }
    }

    fn variable_declaration(&mut self, node: AstNode) {
        if let AstNode::VariableDeclaration { identifier, value } = node {
            if let AstNode::Identifier { name } = *identifier {
                self.expression(*value);

                self.emit(OP_SET_VAR);
                self.emit(self.result.vars.len() as u8);

                self.result.vars.push(Var {
                    name,
                    scope_level: self.scope_level,
                });
            }
        } else {
            panic!("Not a variable declaration");
        }
    }

    fn set_variable(&mut self, node: AstNode) {
        if let AstNode::SetVariable { identifier, value } = node {
            if let AstNode::Identifier { name } = *identifier {
                self.expression(*value);

                self.emit(OP_SET_VAR);
                for i in (0..self.result.vars.len()).rev() {
                    if self.result.vars[i].name == name {
                        self.emit(i as u8);
                        return;
                    }
                }

                panic!("Variable: {} not found", name);
            }
        } else {
            panic!("Not a valid set operation");
        }
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

    fn get_vars_count_on_scope_exit(&mut self) -> u8 {
        let mut count = 0;

        for i in (0..self.result.vars.len()).rev() {
            if self.result.vars[i].scope_level == self.scope_level {
                self.result.vars.pop();
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    fn scope_enter(&mut self) {
        self.scope_level += 1;
    }

    fn scope_exit(&mut self) {
        let vars_count = self.get_vars_count_on_scope_exit();

        self.scope_level -= 1;
        self.emit(OP_SCOPE_EXIT);
        self.emit(vars_count);
    }

    fn emit(&mut self, byte: u8) {
        self.result.bytecode.push(byte);
    }
}
