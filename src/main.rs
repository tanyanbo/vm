mod parser;
mod value;
mod vm;

fn main() {
    let mut virtual_machine = vm::VM::new();
    let result = virtual_machine.exec();

    let source_code = String::from(
        "
              (+ 2 (- 8 2))
            ",
    );

    let mut code_parser = parser::Parser {
        tokenizer: parser::tokenizer::Tokenizer::new(source_code),
    };
    code_parser.parse();
}
