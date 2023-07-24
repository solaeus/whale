use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Partition;

impl BuiltinFunction for Partition {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "disk::partition",
            description: "Partition a disk, clearing its content.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_map()?;
        let path = argument
            .get_value("path")?
            .unwrap_or_default()
            .as_string()?;

        Ok(Value::String(path))
    }
}
