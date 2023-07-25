use crate::{Error, Result, Value};

mod dir;
mod disk;
mod packages;
mod random;
mod system;
mod table;

pub const BUILTIN_FUNCTIONS: [&'static dyn BuiltinFunction; 17] = [
    &dir::Create,
    &dir::Move,
    &dir::Read,
    &dir::Remove,
    &dir::Trash,
    &disk::List,
    &disk::Partition,
    &packages::CoprRepositories,
    &packages::Install,
    &packages::RpmRepositories,
    &packages::Uninstall,
    &packages::Upgrade,
    &random::RandomFloat,
    &random::RandomInteger,
    &random::RandomString,
    &system::SystemCpu,
    &table::Table,
];

pub trait BuiltinFunction: Sync + Send {
    fn info(&self) -> FunctionInfo<'static>;
    fn run(&self, argument: &Value) -> Result<Value>;
}

pub struct FunctionInfo<'a> {
    pub identifier: &'a str,
    pub description: &'a str,
}

pub struct Help;

impl BuiltinFunction for Help {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "help",
            description: "Get help using whale.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;
        println!("{}", include_str!("../../README.md"));
        Ok(Value::Empty)
    }
}

pub fn call_builtin_function(identifier: &str, argument: &Value) -> Result<Value> {
    for function in BUILTIN_FUNCTIONS {
        if identifier == function.info().identifier {
            return function.run(argument);
        }
    }

    Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
}
