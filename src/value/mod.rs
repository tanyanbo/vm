#[derive(Clone, Debug)]
pub enum Value {
    Number { num: f64 },
    String { str: String },
}

pub fn number(num: f64) -> Value {
    Value::Number { num }
}

pub fn string(str: String) -> Value {
    Value::String { str }
}

// pub fn is_number(value: &Value) -> bool {
//     match value {
//         Value::Number { num: _ } => true,
//         _ => false,
//     }
// }

// pub fn is_string(value: &Value) -> bool {
//     match value {
//         Value::String { str: _ } => true,
//         _ => false,
//     }
// }
