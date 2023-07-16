use regex::Regex;

mod parser;
mod value;
mod vm;

fn main() {
    let mut virtual_machine = vm::VM::new();
    let result = virtual_machine.exec();
    parser::Parser::new();

    // println!("Result: {:?}", result);
}
