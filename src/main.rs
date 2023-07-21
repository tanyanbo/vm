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
        (var a 70)
        (begin 
            (var a 10)
            (var b 20)

            (begin 
                (+ a b)
            )
        )
        (begin 
            (var x 10)
            (var b 90)

            (begin 
                (+ a (* 2 x))
            )
        )
        ",
    );

    let mut code_parser = Parser::new(source_code);
    let res = code_parser.parse();

    let mut compiler = Compiler::new(is_debug);
    compiler.compile(res);

    disassembler::disassemble(
        &compiler.result.bytecode,
        &compiler.result.constants,
        &compiler.result.disassembler_vars,
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
