pub mod tokenizer;

enum BinaryExpressionType {
    Add,
    Sub,
    Mul,
    Div,
}

enum AstNode {
    Program {
        children: Vec<AstNode>,
    },
    BinaryExpression {
        r#type: BinaryExpressionType,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    NumberLiteral {
        value: f64,
    },
    StringLiteral {
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
        loop {
            let current_token = self.tokenizer.get_next_token();

            match current_token.kind {
                tokenizer::TokenKind::OpenParen => {
                    self.parse_expr();
                }
                tokenizer::TokenKind::Add => {
                    println!("Add");
                }
                tokenizer::TokenKind::Sub => {
                    println!("Sub");
                }
                tokenizer::TokenKind::Mul => {
                    println!("Mul");
                }
                tokenizer::TokenKind::Div => {
                    println!("Div");
                }
                tokenizer::TokenKind::NumberLiteral => {
                    println!("Number");
                }
                tokenizer::TokenKind::StringLiteral => {
                    println!("Number");
                }
                tokenizer::TokenKind::EndOfFile => {
                    break;
                }
                _ => {
                    panic!("Invalid token");
                }
            }
        }

        if self.tokenizer.get_next_token().kind != tokenizer::TokenKind::CloseParen {
            panic!("Invalid token");
        }
    }
}
