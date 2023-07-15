#[derive(Clone)]
pub enum ValueType {
    NUMBER,
}

#[derive(Clone)]
pub struct Value {
    pub value_type: ValueType,
    pub number: f64,
}

pub fn number(number: f64) -> Value {
    Value {
        value_type: ValueType::NUMBER,
        number: number as f64,
    }
}
