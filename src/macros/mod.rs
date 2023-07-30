use crate::{Error, Result, Value};

pub mod command;
pub mod count;
pub mod data;
pub mod dir;
pub mod disk;
pub mod file;
pub mod git;
pub mod map;
pub mod output;
pub mod packages;
pub mod random;
pub mod sort;
pub mod system;
pub mod table;
pub mod wait;
pub mod whale;

/// Master list of all internal functions.
///
/// This list is used to match identifiers with functions and to provide info
/// to the shell.
pub const MACRO_LIST: [&'static dyn Macro; 44] = [
    &command::Bash,
    &command::Fish,
    &command::Raw,
    &command::Sh,
    &command::Zsh,
    &count::Count,
    &data::Csv,
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
    &git::Status,
    &map::Map,
    &output::Output,
    &packages::CoprRepositories,
    &packages::Install,
    &packages::RpmRepositories,
    &packages::Uninstall,
    &packages::Upgrade,
    &random::RandomFloat,
    &random::RandomInteger,
    &random::RandomString,
    &sort::Sort,
    &system::SystemCpu,
    &system::SystemInfo,
    &table::Create,
    &table::Filter,
    &table::Find,
    &table::Insert,
    &wait::Seconds,
    &wait::Watch,
    &whale::Async,
    &whale::Pipe,
    &whale::Repeat,
    &whale::Run,
];

/// Internal whale function with its business logic and all information.
pub trait Macro: Sync + Send {
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
pub fn call_macro(identifier: &str, argument: &Value) -> Result<Value> {
    for r#macro in MACRO_LIST {
        if identifier == r#macro.info().identifier {
            return r#macro.run(argument);
        }
    }

    Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions_start_with_caps() {
        for function in MACRO_LIST {
            assert!(function
                .info()
                .identifier
                .chars()
                .nth(0)
                .unwrap()
                .is_uppercase())
        }
    }
}
