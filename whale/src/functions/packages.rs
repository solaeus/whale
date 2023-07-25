use std::process::Command;

use crate::{BuiltinFunction, Error, FunctionInfo, Result, Value};

pub struct CoprRepositories;

impl BuiltinFunction for CoprRepositories {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "packages::copr_repositories",
            description: "Enable one or more COPR repositories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let repo_list_string = if let Ok(repo) = argument.as_string() {
            repo
        } else if let Ok(repos) = argument.as_tuple() {
            repos
                .into_iter()
                .map(|value| value.to_string() + " ")
                .collect()
        } else {
            return Err(crate::Error::ExpectedString {
                actual: argument.clone(),
            });
        };

        Command::new("fish")
            .arg("-c")
            .arg(format!("sudo dnf -y copr enable {repo_list_string}"))
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Install;

impl BuiltinFunction for Install {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "packages::install",
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

pub struct RpmRepositories;

impl BuiltinFunction for RpmRepositories {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "packages::rpm_repositories",
            description: "Enable one or more RPM repositories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(repo) = argument.as_string() {
            Command::new("fish")
                .arg("-c")
                .arg(format!("sudo dnf -y config-manager --add-repo {repo}"))
                .spawn()?
                .wait()?;
        } else if let Ok(repos) = argument.as_tuple() {
            for repo in repos {
                Command::new("fish")
                    .arg("-c")
                    .arg(format!("sudo dnf -y config-manager --add-repo {repo}"))
                    .spawn()?
                    .wait()?;
            }
        } else {
            return Err(crate::Error::ExpectedString {
                actual: argument.clone(),
            });
        };

        Ok(Value::Empty)
    }
}

pub struct Uninstall;

impl BuiltinFunction for Uninstall {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "packages::uninstall",
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
            .arg(format!("sudo dnf -y remove {package_list_string}"))
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Upgrade;

impl BuiltinFunction for Upgrade {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "packages::upgrade",
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
