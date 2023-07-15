#[derive(Clone, Debug)]
pub enum Value {
    Number { num: Option<f64> },
    String { str: Option<String> },
}

pub fn number(num: f64) -> Value {
    Value::Number { num: Some(num) }
}

pub fn string(str: String) -> Value {
    Value::String { str: Some(str) }
}

pub fn is_number(value: &Value) -> bool {
    match value {
        Value::Number { num: Some(_) } => true,
        _ => false,
    }
}

pub fn is_string(value: &Value) -> bool {
    match value {
        Value::String { str: Some(_) } => true,
        _ => false,
    }
}
