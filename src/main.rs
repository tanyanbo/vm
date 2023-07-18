use crate::compiler::Compiler;

mod compiler;
mod parser;
mod value;
mod vm;

fn main() {
    let source_code = String::from(
        "
            (+ 2 (- 3 9))
        ",
    );

    let mut code_parser = parser::Parser {
        tokenizer: parser::tokenizer::Tokenizer::new(source_code),
    };
    let res = code_parser.parse();
    // println!("{:#?}", res);

    let mut compiler = Compiler::new();
    compiler.compile(res);
    // println!("{:#?}", compiler.result);

    let mut virtual_machine = vm::VM::new(compiler.result.constants, compiler.result.bytecode);
    let result = virtual_machine.exec();
    println!("{:#?}", result);
}
