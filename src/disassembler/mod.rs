use crate::{compiler::Var, value::Value, vm::*};

pub fn disassemble(bytecode: &Vec<u8>, constants: &Vec<Value>, vars: &Vec<Var>) {
    println!("\n------------------Disassembler--------------------\n");
    let mut ip = 0;
    while ip < bytecode.len() {
        let instruction = bytecode[ip];
        let address: u16 = ip.try_into().unwrap();

        match instruction {
            OP_ADD => disassemble_binary(address, instruction),
            OP_SUB => disassemble_binary(address, instruction),
            OP_MUL => disassemble_binary(address, instruction),
            OP_DIV => disassemble_binary(address, instruction),
            OP_GT => disassemble_binary(address, instruction),
            OP_GTE => disassemble_binary(address, instruction),
            OP_LT => disassemble_binary(address, instruction),
            OP_LTE => disassemble_binary(address, instruction),
            OP_EQ => disassemble_binary(address, instruction),
            OP_CONST => {
                let position = bytecode[ip + 1];
                ip += 1;
                let value = match &constants[position as usize] {
                    Value::Number { val: num } => num.to_string(),
                    Value::String { val: str } => str.clone(),
                    Value::Boolean { val } => val.to_string(),
                };

                dump_bytes(
                    address,
                    vec![OP_CONST, position],
                    instruction,
                    format!("{} ({})", position, value),
                );
            }
            OP_HALT => dump_bytes(address, vec![OP_HALT], instruction, String::from("")),
            OP_JUMP => {
                let position = bytecode[ip + 1];
                ip += 1;
                dump_bytes(
                    address,
                    vec![OP_JUMP, position],
                    instruction,
                    format!("{:04x}", position),
                );
            }
            OP_JUMP_IF_FALSE => {
                let position = bytecode[ip + 1];
                ip += 1;
                dump_bytes(
                    address,
                    vec![OP_JUMP_IF_FALSE, position],
                    instruction,
                    format!("{:04x}", position),
                );
            }
            OP_GET_VAR => {
                let position = bytecode[ip + 1];
                ip += 1;
                dump_bytes(
                    address,
                    vec![OP_GET_VAR, position],
                    instruction,
                    format!("{} ({})", position, vars[position as usize].name),
                );
            }
            OP_SET_VAR => {
                let position = bytecode[ip + 1];
                ip += 1;
                dump_bytes(
                    address,
                    vec![OP_SET_VAR, position],
                    instruction,
                    format!("{} ({})", position, vars[position as usize].name),
                );
            }
            _ => {
                panic!("Invalid instruction");
            }
        }
        ip += 1;
    }
}

fn disassemble_binary(address: u16, instruction: u8) {
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
        OP_JUMP => "JUMP",
        OP_JUMP_IF_FALSE => "JUMP_IF_FALSE",
        OP_CONST => "CONST",
        OP_GET_VAR => "GET_VAR",
        OP_SET_VAR => "SET_VAR",
        _ => {
            panic!("Invalid instruction");
        }
    })
}
