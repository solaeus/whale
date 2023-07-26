use crate::{BuiltinFunction, FunctionInfo, Result, Table, Value};

pub struct Create;

impl BuiltinFunction for Create {
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
            .collect::<Vec<Vec<Value>>>();
        let column_names = argument
            .pop()
            .unwrap_or_default()
            .as_tuple()?
            .iter()
            .map(|value| value.to_string())
            .collect();
        let mut table = Table::new(column_names);

        for row in rows {
            table.insert(row)?;
        }

        Ok(Value::Table(table))
    }
}
