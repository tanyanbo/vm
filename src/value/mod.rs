use crate::compiler::Var;

#[derive(Clone, Debug)]
pub enum Value {
    Number {
        val: f64,
    },
    String {
        val: String,
    },
    Boolean {
        val: bool,
    },
    Function {
        name: String,
        scope_level: u8,
        bytecode: Vec<u8>,
        constants: Vec<Value>,
        vars: Vec<Var>,
        disassembler_vars: Vec<Var>,
    },
}

pub fn number(val: f64) -> Value {
    Value::Number { val }
}

pub fn string(val: String) -> Value {
    Value::String { val }
}

pub fn boolean(val: bool) -> Value {
    Value::Boolean { val }
}
