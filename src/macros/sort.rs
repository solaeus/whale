use crate::{Macro, MacroInfo, Result, Value};

pub struct Sort;

impl Macro for Sort {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "sort",
            description: "Apply default ordering.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(mut list) = argument.as_list().cloned() {
            list.sort();

            Ok(Value::List(list.clone()))
        } else if let Ok(map) = argument.as_map() {
            Ok(Value::Map(map))
        } else if let Ok(mut table) = argument.as_table().cloned() {
            table.sort();

            Ok(Value::Table(table))
        } else {
            Err(crate::Error::ExpectedTuple {
                actual: argument.clone(),
            })
        }
    }
}
