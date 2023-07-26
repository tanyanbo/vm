use self::tokenizer::{CurrentToken, TokenKind, Tokenizer};

pub mod tokenizer;

#[derive(Debug, Clone)]
pub enum LiteralType {
    Number,
    String,
    Boolean,
}

#[derive(Debug, Clone)]
pub enum BinaryExpressionType {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Equal,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Program {
        children: Vec<AstNode>,
    },
    BinaryExpression {
        r#type: BinaryExpressionType,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Literal {
        r#type: LiteralType,
        value: String,
    },
    IfExpression {
        condition: Box<AstNode>,
        consequent: Box<AstNode>,
        alternate: Box<AstNode>,
    },
    WhileExpression {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Identifier {
        name: String,
    },
    VariableDeclaration {
        identifier: Box<AstNode>,
        value: Box<AstNode>,
    },
    SetVariable {
        identifier: Box<AstNode>,
        value: Box<AstNode>,
    },
    Block {
        children: Vec<AstNode>,
    },
    FunctionDeclaration {
        identifier: Box<AstNode>,
        parameters: Vec<AstNode>,
        body: Box<AstNode>,
    },
    CallExpression {
        identifier: Box<AstNode>,
        parameters: Vec<AstNode>,
    },
}

pub struct Parser {
    pub tokenizer: Tokenizer,
    prev_token: Option<CurrentToken>,
    cur_token: Option<CurrentToken>,
}

enum SetVariableType {
    Declare,
    Set,
}

impl Parser {
    pub fn new(source_code: String) -> Parser {
        let source_code_block = format!("(begin {})", source_code);

        Parser {
            tokenizer: Tokenizer::new(source_code_block),
            prev_token: None,
            cur_token: None,
        }
    }

    pub fn parse(&mut self) -> AstNode {
        AstNode::Program {
            children: self.expressions(&TokenKind::EndOfFile),
        }
    }

    fn expressions(&mut self, end_type: &TokenKind) -> Vec<AstNode> {
        let mut expressions: Vec<AstNode> = vec![];
        loop {
            if let Some(cur_token) = self.cur_token.as_ref() {
                if cur_token.kind != TokenKind::OpenParen {
                    self.gen_next_token();

                    if self.cur_token.as_ref().unwrap().kind != TokenKind::OpenParen {
                        panic!("Invalid token");
                    }
                }
            }

            let statement = self.expression();
            expressions.push(statement);

            self.gen_next_token();

            match &self.cur_token.as_ref().unwrap().kind {
                kind if kind == end_type => break expressions,
                TokenKind::OpenParen => {
                    continue;
                }
                TokenKind::CloseParen => {
                    // handle the situation where the last token is an identifier or a literal
                    self.gen_next_token();

                    if &self.cur_token.clone().unwrap().kind == end_type {
                        break expressions;
                    }
                    continue;
                }
                _ => {
                    panic!("Invalid token");
                }
            }
        }
    }

    fn expression(&mut self) -> AstNode {
        self.gen_next_token();

        if let Some(CurrentToken { kind, value }) = self.cur_token.clone() {
            match kind {
                TokenKind::OpenParen => self.expression(),
                TokenKind::Add => self.binary_expression(BinaryExpressionType::Add),
                TokenKind::Sub => self.binary_expression(BinaryExpressionType::Sub),
                TokenKind::Mul => self.binary_expression(BinaryExpressionType::Mul),
                TokenKind::Div => self.binary_expression(BinaryExpressionType::Div),
                TokenKind::Greater => self.binary_expression(BinaryExpressionType::Greater),
                TokenKind::GreaterEqual => {
                    self.binary_expression(BinaryExpressionType::GreaterEqual)
                }
                TokenKind::Lesser => self.binary_expression(BinaryExpressionType::Lesser),
                TokenKind::LesserEqual => self.binary_expression(BinaryExpressionType::LesserEqual),
                TokenKind::Equal => self.binary_expression(BinaryExpressionType::Equal),
                TokenKind::NumberLiteral => self.literal(LiteralType::Number, value),
                TokenKind::StringLiteral => self.literal(LiteralType::String, value),
                TokenKind::BooleanLiteral => self.literal(LiteralType::Boolean, value),
                TokenKind::VariableDeclaration => self.set_variable(SetVariableType::Declare),
                TokenKind::SetVariable => self.set_variable(SetVariableType::Set),
                TokenKind::BeginBlock => self.block(),
                TokenKind::Identifier => self.identifier(value),
                TokenKind::While => self.while_expression(),
                TokenKind::If => self.if_expression(),
                TokenKind::FunctionDeclaration => self.function_declaration(),
                TokenKind::CallFunction => self.call_expression(),
                _ => {
                    panic!("Invalid token");
                }
            }
        } else {
            panic!("No token found");
        }
    }

    fn call_expression(&mut self) -> AstNode {
        let identifier = self.expression();
        let mut parameters = vec![];

        loop {
            if self.tokenizer.lookahead().kind == TokenKind::CloseParen {
                break;
            }
            parameters.push(self.expression());
        }

        self.check_for_close_paren();

        AstNode::CallExpression {
            identifier: Box::new(identifier),
            parameters,
        }
    }

    fn function_declaration(&mut self) -> AstNode {
        let identifier = self.expression();
        let mut parameters = vec![];

        loop {
            if self.tokenizer.lookahead().kind == TokenKind::CloseParen {
                break;
            }
            parameters.push(self.expression());
        }

        self.check_for_close_paren();

        if parameters.iter().any(|param| match param {
            AstNode::Identifier { .. } => false,
            _ => true,
        }) {
            panic!("Invalid parameters");
        }

        let body = self.expression();

        match body {
            AstNode::Block { .. } => {}
            _ => panic!("Invalid body"),
        };

        self.check_for_close_paren();

        AstNode::FunctionDeclaration {
            identifier: Box::new(identifier),
            parameters,
            body: Box::new(body),
        }
    }

    fn while_expression(&mut self) -> AstNode {
        let result = AstNode::WhileExpression {
            condition: Box::new(self.expression()),
            body: Box::new(self.expression()),
        };

        self.check_for_close_paren();

        result
    }

    fn block(&mut self) -> AstNode {
        AstNode::Block {
            children: self.expressions(&TokenKind::CloseParen),
        }
    }

    fn set_variable(&mut self, r#type: SetVariableType) -> AstNode {
        let identifier = self.expression();
        let value = self.expression();

        if let AstNode::Identifier { .. } = identifier {
            self.check_for_close_paren();

            match r#type {
                SetVariableType::Declare => AstNode::VariableDeclaration {
                    identifier: Box::new(identifier),
                    value: Box::new(value),
                },
                SetVariableType::Set => AstNode::SetVariable {
                    identifier: Box::new(identifier),
                    value: Box::new(value),
                },
            }
        } else {
            panic!("Invalid identifier");
        }
    }

    fn identifier(&mut self, name: String) -> AstNode {
        AstNode::Identifier { name }
    }

    fn if_expression(&mut self) -> AstNode {
        let result = AstNode::IfExpression {
            condition: Box::new(self.expression()),
            consequent: Box::new(self.expression()),
            alternate: Box::new(self.expression()),
        };

        self.check_for_close_paren();

        result
    }

    fn binary_expression(&mut self, r#type: BinaryExpressionType) -> AstNode {
        let left = self.expression();
        let right = self.expression();

        self.check_for_close_paren();

        AstNode::BinaryExpression {
            r#type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn literal(&mut self, r#type: LiteralType, value: String) -> AstNode {
        AstNode::Literal { r#type, value }
    }

    fn check_for_close_paren(&mut self) {
        if self.tokenizer.get_next_token().kind != TokenKind::CloseParen {
            panic!("Invalid token");
        }
    }

    fn gen_next_token(&mut self) {
        self.prev_token = self.cur_token.clone();
        self.cur_token = Some(self.tokenizer.get_next_token());
    }
}
