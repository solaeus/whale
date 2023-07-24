use std::process::Command;

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
        let label = argument
            .get_value("label")?
            .unwrap_or_default()
            .as_string()?;
        let name = argument
            .get_value("name")?
            .unwrap_or_default()
            .as_string()?;
        let filesystem = argument
            .get_value("filesystem")?
            .unwrap_or_default()
            .as_string()?;
        let mut range = argument
            .get_value("range")?
            .unwrap_or_default()
            .as_tuple()?;

        if range.len() != 2 {
            return Err(crate::Error::ExpectedFixedLenTuple {
                expected_len: 2,
                actual: Value::Tuple(range),
            });
        }

        let range_end = range.pop().unwrap_or_default().as_string()?;
        let range_start = range.pop().unwrap_or_default().as_string()?;

        let script = format!(
            "sudo parted {path} mklabel {label} mkpart {name} {filesystem} {range_start} {range_end}"
        );

        Command::new("fish")
            .arg("-c")
            .arg(&script)
            .spawn()?
            .wait()?;

        Ok(Value::String(path))
    }
}
