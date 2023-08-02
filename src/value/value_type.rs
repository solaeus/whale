use crate::Value;

/// The type of a `Value`.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ValueType {
    String,
    Float,
    Int,
    Boolean,
    List,
    Empty,
    Map,
    Table,
    Function,
}

impl From<&Value> for ValueType {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(_) => ValueType::String,
            Value::Float(_) => ValueType::Float,
            Value::Integer(_) => ValueType::Int,
            Value::Boolean(_) => ValueType::Boolean,
            Value::List(_) => ValueType::List,
            Value::Empty => ValueType::Empty,
            Value::Map(_) => ValueType::Map,
            Value::Table { .. } => ValueType::Table,
            Value::Function(_) => ValueType::Function,
        }
    }
}

impl From<&mut Value> for ValueType {
    fn from(value: &mut Value) -> Self {
        From::<&Value>::from(value)
    }
}

impl From<&&mut Value> for ValueType {
    fn from(value: &&mut Value) -> Self {
        From::<&Value>::from(*value)
    }
}
