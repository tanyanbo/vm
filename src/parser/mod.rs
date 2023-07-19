use self::tokenizer::{CurrentToken, TokenKind, Tokenizer};

pub mod tokenizer;

#[derive(Debug)]
pub enum LiteralType {
    Number,
    String,
}

#[derive(Debug)]
pub enum BinaryExpressionType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum ComparisonType {
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
    ComparisonExpression {
        r#type: ComparisonType,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Literal {
        r#type: LiteralType,
        value: String,
    },
    IfStatement {
        condition: Box<AstNode>,
        consequent: Box<AstNode>,
        alternate: Box<AstNode>,
    },
}

pub struct Parser {
    pub tokenizer: Tokenizer,
}

impl Parser {
    pub fn parse(&mut self) -> AstNode {
        let mut statements: Vec<AstNode> = vec![];
        loop {
            let current_token = self.tokenizer.get_next_token();

            if current_token.kind != TokenKind::OpenParen {
                panic!("Invalid token");
            }

            let statement = self.parse_expr();
            statements.push(statement);

            let k = self.tokenizer.get_next_token().kind;
            match k {
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

    fn parse_expr(&mut self) -> AstNode {
        let current_token = self.tokenizer.get_next_token();

        match current_token.kind {
            TokenKind::OpenParen => self.parse_expr(),
            TokenKind::Add => self.parse_binary_expression(BinaryExpressionType::Add),
            TokenKind::Sub => self.parse_binary_expression(BinaryExpressionType::Sub),
            TokenKind::Mul => self.parse_binary_expression(BinaryExpressionType::Mul),
            TokenKind::Div => self.parse_binary_expression(BinaryExpressionType::Div),
            TokenKind::NumberLiteral => self.parse_literal(LiteralType::Number, current_token),
            TokenKind::StringLiteral => self.parse_literal(LiteralType::String, current_token),
            TokenKind::If => self.parse_if(),
            TokenKind::Greater => self.parse_comparison(ComparisonType::Greater),
            TokenKind::GreaterEqual => self.parse_comparison(ComparisonType::GreaterEqual),
            TokenKind::Lesser => self.parse_comparison(ComparisonType::Lesser),
            TokenKind::LesserEqual => self.parse_comparison(ComparisonType::LesserEqual),
            TokenKind::Equal => self.parse_comparison(ComparisonType::Equal),
            _ => {
                panic!("Invalid token");
            }
        }
    }

    fn parse_if(&mut self) -> AstNode {
        AstNode::IfStatement {
            condition: Box::new(self.parse_expr()),
            consequent: Box::new(self.parse_expr()),
            alternate: Box::new(self.parse_expr()),
        }
    }

    fn parse_comparison(&mut self, r#type: ComparisonType) -> AstNode {
        let left = self.parse_expr();
        let right = self.parse_expr();

        self.check_for_close_paren();

        AstNode::ComparisonExpression {
            r#type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn parse_binary_expression(&mut self, r#type: BinaryExpressionType) -> AstNode {
        let left = self.parse_expr();
        let right = self.parse_expr();

        self.check_for_close_paren();

        AstNode::BinaryExpression {
            r#type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn parse_literal(&mut self, r#type: LiteralType, current_token: CurrentToken) -> AstNode {
        AstNode::Literal {
            r#type,
            value: current_token.value,
        }
    }

    fn check_for_close_paren(&mut self) {
        if self.tokenizer.get_next_token().kind != TokenKind::CloseParen {
            panic!("Invalid token");
        }
    }
}
