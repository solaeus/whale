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

/// Master list of all internal functions.
///
/// This list is used to match identifiers with functions and to provide info
/// to the shell.
pub const BUILTIN_FUNCTIONS: [&'static dyn BuiltinFunction; 35] = [
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
    &system::SystemInfo,
    &table::Create,
    &table::Find,
    &table::Insert,
    &whale::Async,
    &whale::Repeat,
    &whale::Run,
    &whale::RunFile,
];

/// Internal whale function with its business logic and all information.
pub trait BuiltinFunction: Sync + Send {
    fn info(&self) -> FunctionInfo<'static>;
    fn run(&self, argument: &Value) -> Result<Value>;
}

/// Information needed for each function.
pub struct FunctionInfo<'a> {
    /// Text pattern that triggers this function.
    pub identifier: &'a str,

    /// User-facing information about how the function works.
    pub description: &'a str,
}

/// Searches all functions for a matching identifier and runs the corresponding
/// function with the given input. Returns the function's output or an error.
pub fn call_builtin_function(identifier: &str, argument: &Value) -> Result<Value> {
    for function in BUILTIN_FUNCTIONS {
        if identifier == function.info().identifier {
            return function.run(argument);
        }
    }

    Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
}
