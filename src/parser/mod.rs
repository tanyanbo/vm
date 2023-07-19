use self::tokenizer::{TokenKind, Tokenizer};

pub mod tokenizer;

#[derive(Debug)]
pub enum LiteralType {
    Number,
    String,
    Boolean,
}

#[derive(Debug)]
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

#[derive(Debug)]
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
}

pub struct Parser {
    pub tokenizer: Tokenizer,
    prev_token_kind: Option<TokenKind>,
    cur_token_kind: Option<TokenKind>,
}

impl Parser {
    pub fn new(source_code: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(source_code),
            prev_token_kind: None,
            cur_token_kind: None,
        }
    }

    pub fn parse(&mut self) -> AstNode {
        let mut statements: Vec<AstNode> = vec![];
        loop {
            let current_token = self.tokenizer.get_next_token();

            if current_token.kind != TokenKind::OpenParen {
                panic!("Invalid token");
            }

            let statement = self.expression();
            statements.push(statement);

            match self.tokenizer.get_next_token().kind {
                TokenKind::EndOfFile => {
                    break AstNode::Program {
                        children: statements,
                    };
                }
                TokenKind::OpenParen => {
                    continue;
                }
                _ => {
                    panic!("Invalid token");
                }
            }
        }
    }

    fn expression(&mut self) -> AstNode {
        self.prev_token_kind = self.cur_token_kind.clone();
        let current_token = self.tokenizer.get_next_token();
        self.cur_token_kind = Some(current_token.kind.clone());

        let res = match current_token.kind {
            TokenKind::OpenParen => self.expression(),
            TokenKind::Add => self.binary_expression(BinaryExpressionType::Add),
            TokenKind::Sub => self.binary_expression(BinaryExpressionType::Sub),
            TokenKind::Mul => self.binary_expression(BinaryExpressionType::Mul),
            TokenKind::Div => self.binary_expression(BinaryExpressionType::Div),
            TokenKind::Greater => self.binary_expression(BinaryExpressionType::Greater),
            TokenKind::GreaterEqual => self.binary_expression(BinaryExpressionType::GreaterEqual),
            TokenKind::Lesser => self.binary_expression(BinaryExpressionType::Lesser),
            TokenKind::LesserEqual => self.binary_expression(BinaryExpressionType::LesserEqual),
            TokenKind::Equal => self.binary_expression(BinaryExpressionType::Equal),
            TokenKind::NumberLiteral => self.literal(LiteralType::Number, current_token.value),
            TokenKind::StringLiteral => self.literal(LiteralType::String, current_token.value),
            TokenKind::BooleanLiteral => self.literal(LiteralType::Boolean, current_token.value),
            TokenKind::If => self.if_expression(),
            _ => {
                panic!("Invalid token");
            }
        };

        res
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
        if self.prev_token_kind == Some(TokenKind::OpenParen) {
            self.check_for_close_paren();
        }

        AstNode::Literal { r#type, value }
    }

    fn check_for_close_paren(&mut self) {
        if self.tokenizer.get_next_token().kind != TokenKind::CloseParen {
            panic!("Invalid token");
        }
    }
}
