use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Sort;

impl BuiltinFunction for Sort {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "sort",
            description: "Apply default ordering.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(mut list) = argument.as_list() {
            list.sort();

            Ok(Value::List(list))
        } else if let Ok(map) = argument.as_map() {
            Ok(Value::Map(map))
        } else if let Ok(mut table) = argument.as_table() {
            table.sort();

            Ok(Value::Table(table))
        } else {
            Err(crate::Error::ExpectedTuple {
                actual: argument.clone(),
            })
        }
    }
}
