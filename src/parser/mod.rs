pub mod tokenizer;

pub struct Parser {
    pub tokenizer: tokenizer::Tokenizer,
}

impl Parser {
    pub fn parse(&mut self) {
        loop {
            let current_token = self.tokenizer.get_next_token();
            println!("{:?}", current_token);
            if current_token.kind == tokenizer::TokenKind::EndOfFile {
                break;
            }
        }
        // match current_token.kind {
        //     tokenizer::TokenKind::OpenParen => {
        //         println!("OpenParen");
        //     }
        //     tokenizer::TokenKind::CloseParen => {
        //         println!("CloseParen");
        //     }
        //     tokenizer::TokenKind::Add => {
        //         println!("Add");
        //     }
        //     tokenizer::TokenKind::Sub => {
        //         println!("Sub");
        //     }
        //     tokenizer::TokenKind::Mul => {
        //         println!("Mul");
        //     }
        //     tokenizer::TokenKind::Div => {
        //         println!("Div");
        //     }
        //     tokenizer::TokenKind::NumberLiteral => {
        //         println!("Number");
        //     }
        //     tokenizer::TokenKind::StringLiteral => {
        //         println!("Number");
        //     }
        //     _ => {}
        // }
    }
}
