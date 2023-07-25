use std::process::Command;

use crate::{BuiltinFunction, Error, FunctionInfo, Result, Value};

pub struct Install;

impl BuiltinFunction for Install {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "package::install",
            description: "Install one or more packages.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let package_list_string = if let Ok(package) = argument.as_string() {
            package
        } else if let Ok(packages) = argument.as_tuple() {
            packages
                .into_iter()
                .map(|value| value.to_string() + " ")
                .collect()
        } else {
            return Err(Error::ExpectedString {
                actual: argument.clone(),
            });
        };

        Command::new("fish")
            .arg("-c")
            .arg(format!("sudo dnf -y install {package_list_string}"))
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Uninstall;

impl BuiltinFunction for Uninstall {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "package::uninstall",
            description: "Uninstall one or more packages.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let package_list_string = if let Ok(package) = argument.as_string() {
            package
        } else if let Ok(packages) = argument.as_tuple() {
            packages
                .into_iter()
                .map(|value| value.to_string() + " ")
                .collect()
        } else {
            return Err(Error::ExpectedString {
                actual: argument.clone(),
            });
        };

        Command::new("fish")
            .arg("-c")
            .arg(format!("sudo dnf -y uninstall {package_list_string}"))
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

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
