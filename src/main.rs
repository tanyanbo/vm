use self::value::Value;
use parser::Parser;

use crate::compiler::Compiler;

mod compiler;
mod disassembler;
mod parser;
mod value;
mod vm;

fn main() {
    let is_debug = true;
    let source_code = String::from(
        "
        (

            (def test (x y) (begin
                (+ x y)
            ))
            (call test 10 20)
        )
        ",
    );

    let mut code_parser = Parser::new(source_code);
    let res = code_parser.parse();

    let mut compiler = Compiler::new(is_debug);
    compiler.compile(res);

    // disassembler::disassemble(
    //     &compiler.result.bytecode,
    //     &compiler.result.constants,
    //     &compiler.result.disassembler_vars,
    // );

    let mut virtual_machine = vm::VM::new();
    let result = virtual_machine.exec(compiler.result.constants, compiler.result.bytecode);
    match &result {
        Value::Number { val: num } => {
            println!("\nResult: {}", num);
        }
        Value::String { val: str } => {
            println!("\nResult: {}", str);
        }
        Value::Boolean { val } => {
            println!("\nResult: {}", val);
        }
        Value::Function { name, .. } => {
            println!("\nResult: (function) {}", name);
        }
    }
}
