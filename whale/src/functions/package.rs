use std::process::Command;

use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Upgrade;

impl BuiltinFunction for Upgrade {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "package::upgrade",
            description: "Upgrade all installed packages.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        Command::new("fish")
            .arg("-c")
            .arg("sudo dnf -y upgrade")
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}
