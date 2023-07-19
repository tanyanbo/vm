#[derive(Clone, Debug)]
pub enum Value {
    Number { num: f64 },
    String { str: String },
    Boolean { val: bool },
}

pub fn number(num: f64) -> Value {
    Value::Number { num }
}

pub fn string(str: String) -> Value {
    Value::String { str }
}

pub fn boolean(val: bool) -> Value {
    Value::Boolean { val }
}
