pub mod tokenizer;

pub struct Parser {
    tokenizer: tokenizer::Tokenizer,
}

impl Parser {
    pub fn new() -> Parser {
        let source_code = String::from(
            "
              (+ 2 (- 8 2))
            ",
        );
        let mut tokenizer = tokenizer::Tokenizer::new(source_code);
        let x = tokenizer.get_next_token();
        println!("{:?}", x);

        Parser { tokenizer }
    }
}
