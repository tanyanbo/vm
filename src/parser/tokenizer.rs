use regex::Regex;

#[derive(Clone, Debug)]
enum TokenKind {
    OpenParen,
    CloseParen,
    Add,
    Sub,
    Mul,
    Div,
    NumberLiteral,
    StringLiteral,
    Whitespace,
}

struct Token {
    kind: TokenKind,
    test: Regex,
}

const NUMBER_OF_TOKENS: usize = 9;

pub struct Tokenizer {
    input: String,
    cursor: usize,
    tokens: [Token; NUMBER_OF_TOKENS],
}

#[derive(Debug)]
pub struct CurrentToken {
    kind: TokenKind,
    value: String,
}

impl Tokenizer {
    pub fn get_next_token(&mut self) -> CurrentToken {
        self.input = self.input[self.cursor..].to_string();
        println!("input: {}", self.input);

        for token in self.tokens.iter() {
            if let Some(captures) = token.test.captures(&self.input) {
                let result = captures.get(0).unwrap().as_str();

                let length = result.len();
                self.cursor += length;

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
                    kind: TokenKind::Whitespace,
                    test: Regex::new(r"^\s+").unwrap(),
                },
            ],
        }
    }
}
