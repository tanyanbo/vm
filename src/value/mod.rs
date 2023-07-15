#[derive(Clone, PartialEq)]
pub enum ValueType {
    NUMBER,
}

#[derive(Clone)]
pub struct Value {
    value_type: ValueType,
    pub number: f64,
}

pub fn number(number: f64) -> Value {
    Value {
        value_type: ValueType::NUMBER,
        number: number as f64,
    }
}

pub fn is_number(value: &Value) -> bool {
    value.value_type == ValueType::NUMBER
}
