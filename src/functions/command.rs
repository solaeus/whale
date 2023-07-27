use std::process::Command;

use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Sh;

impl BuiltinFunction for Sh {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::sh",
            description: "Pass input to the Bourne Shell.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new("sh").arg("-c").arg(argument).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}

pub struct Bash;

impl BuiltinFunction for Bash {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::bash",
            description: "Pass input to the Bourne Again Shell.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new("bash")
            .arg("-c")
            .arg(argument)
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}
pub struct Fish;

impl BuiltinFunction for Fish {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::fish",
            description: "Pass input to the fish shell.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new("fish")
            .arg("-c")
            .arg(argument)
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Zsh;

impl BuiltinFunction for Zsh {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::zsh",
            description: "Pass input to the Z shell.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new("zsh")
            .arg("-c")
            .arg(argument)
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Raw;

impl BuiltinFunction for Raw {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "command::raw",
            description: "Run input as a command without a shell",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new(argument).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}
