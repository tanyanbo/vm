#[derive(Clone, Debug)]
pub enum Value {
    Number { val: f64 },
    String { val: String },
    Boolean { val: bool },
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
