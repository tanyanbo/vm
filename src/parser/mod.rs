use self::tokenizer::CurrentToken;

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
}

pub struct Parser {
    pub tokenizer: tokenizer::Tokenizer,
}

impl Parser {
    pub fn parse(&mut self) -> AstNode {
        let mut statements: Vec<AstNode> = vec![];
        loop {
            let mut current_token = self.tokenizer.get_next_token();

            if current_token.kind != tokenizer::TokenKind::OpenParen {
                panic!("Invalid token");
            }

            let statement = self.parse_expr();
            statements.push(statement);

            current_token = self.tokenizer.get_next_token();

            match current_token.kind {
                tokenizer::TokenKind::EndOfFile => {
                    break AstNode::Program {
                        children: statements,
                    };
                }
                tokenizer::TokenKind::OpenParen => {
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

        return match current_token.kind {
            tokenizer::TokenKind::OpenParen => self.parse_expr(),
            tokenizer::TokenKind::Add => self.parse_binary_expression(BinaryExpressionType::Add),
            tokenizer::TokenKind::Sub => self.parse_binary_expression(BinaryExpressionType::Sub),
            tokenizer::TokenKind::Mul => self.parse_binary_expression(BinaryExpressionType::Mul),
            tokenizer::TokenKind::Div => self.parse_binary_expression(BinaryExpressionType::Div),
            tokenizer::TokenKind::NumberLiteral => {
                self.parse_literal(LiteralType::Number, current_token)
            }
            tokenizer::TokenKind::StringLiteral => {
                self.parse_literal(LiteralType::String, current_token)
            }
            _ => {
                panic!("Invalid token");
            }
        };
    }

    fn parse_binary_expression(&mut self, r#type: BinaryExpressionType) -> AstNode {
        let left = self.parse_expr();
        let right = self.parse_expr();

        if self.tokenizer.get_next_token().kind != tokenizer::TokenKind::CloseParen {
            panic!("Invalid token");
        }

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
}
