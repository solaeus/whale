use crate::{FunctionInfo, Macro, Result, Value};

pub struct Count;

impl Macro for Count {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "count",
            description: "Return the number of items in a collection.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let len = match argument {
            Value::String(string) => string.len(),
            Value::List(list) => list.len(),
            Value::Map(map) => map.len(),
            Value::Table(table) => table.len(),
            Value::Function(_) | Value::Float(_) | Value::Integer(_) | Value::Boolean(_) => 1,
            Value::Empty => 0,
        };

        Ok(Value::Integer(len as i64))
    }
}
