use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Table;

impl BuiltinFunction for Table {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "table",
            description: "Define a new table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut argument = argument.as_tuple()?;
        let rows = argument
            .pop()
            .unwrap_or_default()
            .as_tuple()?
            .iter()
            .map(|value| value.as_tuple().unwrap_or_default())
            .collect();
        let column_names = argument
            .pop()
            .unwrap_or_default()
            .as_tuple()?
            .iter()
            .map(|value| value.to_string())
            .collect();

        Ok(Value::Table { column_names, rows })
    }
}
