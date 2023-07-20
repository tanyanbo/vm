use parser::Parser;

use crate::compiler::Compiler;

mod compiler;
mod disassembler;
mod parser;
mod value;
mod vm;

fn main() {
    let source_code = String::from(
        "
        (var y 5)
        (var x 2)
        (var z (+ x y))
        ",
    );

    let mut code_parser = Parser::new(source_code);
    let res = code_parser.parse();

    let mut compiler = Compiler::new();
    compiler.compile(res);

    disassembler::disassemble(
        &compiler.result.bytecode,
        &compiler.result.constants,
        &compiler.result.vars,
    );

    let mut virtual_machine = vm::VM::new(compiler.result.constants, compiler.result.bytecode);
    let result = virtual_machine.exec();
    match &result {
        value::Value::Number { val: num } => {
            println!("\nResult: {}", num);
        }
        value::Value::String { val: str } => {
            println!("\nResult: {}", str);
        }
        value::Value::Boolean { val } => {
            println!("\nResult: {}", val);
        }
    }
}
