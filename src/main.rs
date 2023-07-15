mod value;
mod vm;

fn main() {
    let mut virtual_machine = vm::VM::new();
    let result = virtual_machine.exec();
    println!("Result: {}", result.number);
}
