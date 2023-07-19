use crate::{value::Value, vm::*};

pub fn disassemble(bytecode: &Vec<u8>, constants: &Vec<Value>) {
    println!("\n------------------Disassembler--------------------\n");
    let mut ip = 0;
    while ip < bytecode.len() {
        let instruction = bytecode[ip];
        let address: u16 = ip.try_into().unwrap();

        match instruction {
            OP_ADD => disassemble_math(address, instruction),
            OP_SUB => disassemble_math(address, instruction),
            OP_MUL => disassemble_math(address, instruction),
            OP_DIV => disassemble_math(address, instruction),
            OP_GT => disassemble_math(address, instruction),
            OP_GTE => disassemble_math(address, instruction),
            OP_LT => disassemble_math(address, instruction),
            OP_LTE => disassemble_math(address, instruction),
            OP_EQ => disassemble_math(address, instruction),
            OP_CONST => {
                let position = bytecode[ip + 1];
                let value = match &constants[position as usize] {
                    Value::Number { num } => num.to_string(),
                    Value::String { str } => str.clone(),
                    Value::Boolean { val } => val.to_string(),
                };

                dump_bytes(
                    address,
                    vec![OP_CONST, position],
                    instruction,
                    format!("{} ({})", position, value),
                );
                ip += 1;
            }
            OP_HALT => dump_bytes(address, vec![OP_HALT], instruction, String::from("")),
            _ => {
                panic!("Invalid instruction");
            }
        }
        ip += 1;
    }
}

fn disassemble_math(address: u16, instruction: u8) {
    dump_bytes(address, vec![instruction], instruction, String::from(""))
}

fn dump_bytes(address: u16, bytes: Vec<u8>, instruction: u8, info: String) {
    let bytes_string = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(" ");

    println!(
        "{:04x}      {:<14}{:<20}{}",
        address,
        bytes_string,
        op_code_name(instruction),
        info
    );
}

fn op_code_name(op_code: u8) -> String {
    String::from(match op_code {
        OP_ADD => "ADD",
        OP_SUB => "SUB",
        OP_MUL => "MUL",
        OP_DIV => "DIV",
        OP_GT => "GT",
        OP_GTE => "GTE",
        OP_LT => "LT",
        OP_LTE => "LTE",
        OP_EQ => "EQ",
        OP_HALT => "HALT",
        OP_CONST => "CONST",
        _ => {
            panic!("Invalid instruction");
        }
    })
}
