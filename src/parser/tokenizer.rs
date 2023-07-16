use regex::Regex;

enum TokenKind {
    OpenParen,
    CloseParen,
}

struct Token {
    kind: TokenKind,
    test: Regex,
}

const NUMBER_OF_TOKENS: usize = 1;

pub struct Tokenizer {
    input: String,
    cursor: usize,
    tokens: [Token; NUMBER_OF_TOKENS],
}

impl Tokenizer {
    fn new(&self, input: String) -> Tokenizer {
        Tokenizer {
            input,
            cursor: 0,
            tokens: [Token {
                kind: TokenKind::OpenParen,
                test: Regex::new(r"(").unwrap(),
            }],
        }
    }
}
