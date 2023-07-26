use std::process::Command;

use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Sh;

impl BuiltinFunction for Sh {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::sh",
            description: "Pass input to the bourne shell.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new("sh").arg("-c").arg(argument).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}
