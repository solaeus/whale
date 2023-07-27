use crate::{Error, Result, Value};

mod command;
mod dir;
mod disk;
mod file;
mod packages;
mod random;
mod system;
mod table;
mod whale;

pub const BUILTIN_FUNCTIONS: [&'static dyn BuiltinFunction; 31] = [
    &command::Bash,
    &command::Fish,
    &command::Raw,
    &command::Sh,
    &command::Zsh,
    &dir::Create,
    &dir::Move,
    &dir::Read,
    &dir::Remove,
    &dir::Trash,
    &disk::List,
    &disk::Partition,
    &file::Convert,
    &file::FileAppend,
    &file::Metadata,
    &file::Read,
    &file::Remove,
    &file::Write,
    &packages::CoprRepositories,
    &packages::Install,
    &packages::RpmRepositories,
    &packages::Uninstall,
    &packages::Upgrade,
    &random::RandomFloat,
    &random::RandomInteger,
    &random::RandomString,
    &system::SystemCpu,
    &table::Create,
    &table::Insert,
    &whale::Run,
    &whale::Async,
];

pub trait BuiltinFunction: Sync + Send {
    fn info(&self) -> FunctionInfo<'static>;
    fn run(&self, argument: &Value) -> Result<Value>;
}

pub struct FunctionInfo<'a> {
    pub identifier: &'a str,
    pub description: &'a str,
}

pub fn call_builtin_function(identifier: &str, argument: &Value) -> Result<Value> {
    for function in BUILTIN_FUNCTIONS {
        if identifier == function.info().identifier {
            return function.run(argument);
        }
    }

    Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
}
