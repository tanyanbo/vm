use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Parens
    OpenParen,
    CloseParen,

    // Math
    Add,
    Sub,
    Mul,
    Div,

    // Literals
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,

    // Control flow
    If,

    // Comparison
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    Equal,

    // Special
    Whitespace,
    EndOfFile,
}

struct Token {
    kind: TokenKind,
    test: Regex,
}

const NUMBER_OF_TOKENS: usize = 16;

pub struct Tokenizer {
    input: String,
    cursor: usize,
    tokens: [Token; NUMBER_OF_TOKENS],
}

#[derive(Debug)]
pub struct CurrentToken {
    pub kind: TokenKind,
    pub value: String,
}

impl Tokenizer {
    pub fn get_next_token(&mut self) -> CurrentToken {
        if self.cursor >= self.input.len() {
            return CurrentToken {
                kind: TokenKind::EndOfFile,
                value: "".to_string(),
            };
        }

        self.input = self.input[self.cursor..].to_string();

        for token in self.tokens.iter() {
            if let Some(captures) = token.test.captures(&self.input) {
                let result = captures.get(0).unwrap().as_str();

                let length = result.len();
                self.cursor = length;

                if token.kind == TokenKind::Whitespace {
                    return self.get_next_token();
                }

                return CurrentToken {
                    kind: token.kind.clone(),
                    value: result.to_string(),
                };
            }
        }

        panic!("Invalid token");
    }

    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            cursor: 0,
            tokens: [
                Token {
                    kind: TokenKind::OpenParen,
                    test: Regex::new(r"^\(").unwrap(),
                },
                Token {
                    kind: TokenKind::CloseParen,
                    test: Regex::new(r"^\)").unwrap(),
                },
                Token {
                    kind: TokenKind::Add,
                    test: Regex::new(r"^\+").unwrap(),
                },
                Token {
                    kind: TokenKind::Sub,
                    test: Regex::new(r"^-").unwrap(),
                },
                Token {
                    kind: TokenKind::Mul,
                    test: Regex::new(r"^\*").unwrap(),
                },
                Token {
                    kind: TokenKind::Div,
                    test: Regex::new(r"^/").unwrap(),
                },
                Token {
                    kind: TokenKind::NumberLiteral,
                    test: Regex::new(r"^\d+").unwrap(),
                },
                Token {
                    kind: TokenKind::StringLiteral,
                    test: Regex::new("^\".*\"").unwrap(),
                },
                Token {
                    kind: TokenKind::BooleanLiteral,
                    test: Regex::new("^(true|false)").unwrap(),
                },
                Token {
                    kind: TokenKind::If,
                    test: Regex::new("^if").unwrap(),
                },
                Token {
                    kind: TokenKind::Whitespace,
                    test: Regex::new(r"^\s+").unwrap(),
                },
                Token {
                    kind: TokenKind::Greater,
                    test: Regex::new(r"^>").unwrap(),
                },
                Token {
                    kind: TokenKind::GreaterEqual,
                    test: Regex::new(r"^>=").unwrap(),
                },
                Token {
                    kind: TokenKind::Lesser,
                    test: Regex::new("^<").unwrap(),
                },
                Token {
                    kind: TokenKind::LesserEqual,
                    test: Regex::new("^<=").unwrap(),
                },
                Token {
                    kind: TokenKind::Equal,
                    test: Regex::new(r"^=").unwrap(),
                },
            ],
        }
    }
}
