//! This module contains whale's built-in macro system. Every macro is listed
//! alphabetically. Use [call_macro] to check an identifier against every macro.
//!
//! ## Writing macros
//!
//! - snake case identifier, this is enforced by a test
//! - the type name should be the identifier in upper camel case
//! - always verify user input, this creates helpful errors
//! - the description should be brief, it will display in the shell
//! - maintain alphabetical order
//!
//! ## Usage
//!
//! Macros can be used in Rust by passing a Value to the run method.
//!
//! ```
//! let value = Value::List(vec![1, 2,3]);
//! let count = Count.run(value).as_string().unwrap();
//!
//! assert_eq!(count, 3);
//! ```
use crate::{Result, Value};

mod collections;
mod command;
mod data_formats;
mod disks;
mod filesystem;
mod general;
mod gui;
mod logic;
mod network;
mod package_management;
mod random;
mod test;

/// Master list of all macros.
///
/// This list is used to match identifiers with macros and to provide info to
/// the shell.
pub const MACRO_LIST: [&'static dyn Macro; 42] = [
    &command::Bash,
    &command::Fish,
    &command::Raw,
    &command::Sh,
    &command::Zsh,
    &data_formats::FromJson,
    &filesystem::Append,
    &filesystem::CreateDir,
    &filesystem::FileMetadata,
    &filesystem::MoveDir,
    &filesystem::ReadDir,
    &filesystem::ReadFile,
    &filesystem::RemoveDir,
    &filesystem::Trash,
    &filesystem::Watch,
    &filesystem::Write,
    &general::Output,
    &general::Async,
    &general::Repeat,
    &general::Run,
    &general::Wait,
    &gui::Gui,
    &collections::CreateTable,
    &collections::Get,
    &collections::Insert,
    &collections::Where,
    &collections::Select,
    &test::Assert,
    &test::AssertEqual,
    &logic::If,
    &network::Download,
    &random::RandomBoolean,
    &random::RandomFloat,
    &random::RandomInteger,
    &random::RandomString,
    &package_management::CoprRepositories,
    &package_management::EnableRpmRepositories,
    &package_management::InstallPackage,
    &package_management::UninstallPackage,
    &package_management::UpgradePackages,
    &disks::ListDisks,
    &disks::Partition,
];

/// A whale macro function.
pub trait Macro: Sync + Send {
    fn info(&self) -> MacroInfo<'static>;
    fn run(&self, argument: &Value) -> Result<Value>;
}

/// Information needed for each macro.
pub struct MacroInfo<'a> {
    /// Text pattern that triggers this macro.
    pub identifier: &'a str,

    /// User-facing information about how the macro works.
    pub description: &'a str,

    /// Category used to sort macros in the shell.
    pub group: &'a str,
}

// pub struct Pipe;

// impl Macro for Pipe {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "pipe",
//             description: "Process a value with a list of functions.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let argument_list = argument.as_list()?;
//         let input = &argument_list[0];
//         let pipe = &argument_list[1..];
//         let mut accumulator = input.clone();

//         for value in pipe {
//             accumulator = value.as_function()?.run()?;
//         }

//         Ok(accumulator)
//     }
// }

// pub struct SystemInfo;

// impl Macro for SystemInfo {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "system_info",
//             description: "Get information on the system.",
//         }
//     }

//     fn run(&self, argument: &Value) -> crate::Result<Value> {
//         argument.as_empty()?;

//         let mut map = VariableMap::new();

//         map.set_value("hostname", Value::String(hostname()?))?;

//         Ok(Value::Map(map))
//     }
// }

// pub struct SystemCpu;

// impl Macro for SystemCpu {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "system_cpu",
//             description: "Get information on the system's CPU.",
//         }
//     }

//     fn run(&self, argument: &Value) -> crate::Result<Value> {
//         argument.as_empty()?;

//         let mut table = Table::new(vec!["count".to_string(), "speed".to_string()]);
//         let count = cpu_num().unwrap_or_default() as i64;
//         let speed = cpu_speed().unwrap_or_default() as i64;

//         table.insert(vec![Value::Integer(count), Value::Integer(speed)])?;

//         Ok(Value::Table(table))
//     }
// }

// pub struct Sort;

// impl Macro for Sort {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "sort",
//             description: "Apply default ordering.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         if let Ok(mut list) = argument.as_list().cloned() {
//             list.sort();

//             Ok(Value::List(list))
//         } else if let Ok(map) = argument.as_map().cloned() {
//             Ok(Value::Map(map))
//         } else if let Ok(mut table) = argument.as_table().cloned() {
//             table.sort();

//             Ok(Value::Table(table))
//         } else {
//             Err(crate::Error::ExpectedList {
//                 actual: argument.clone(),
//             })
//         }
//     }
// }

// pub struct Map;

// impl Macro for Map {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "map",
//             description: "Create a map from a value.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         match argument {
//             Value::String(_) => todo!(),
//             Value::Float(_) => todo!(),
//             Value::Integer(_) => todo!(),
//             Value::Boolean(_) => todo!(),
//             Value::List(_) => todo!(),
//             Value::Map(_) => todo!(),
//             Value::Table(table) => Ok(Value::Map(VariableMap::from(table))),
//             Value::Function(_) => todo!(),
//             Value::Empty => todo!(),
//         }
//     }
// }

// pub struct Transform;

// impl Macro for Transform {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "transform",
//             description: "Change each value with a function.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let argument = argument.as_list()?;
//         let value = &argument[0];
//         let function = argument[1].as_function()?;

//         match value {
//             Value::String(_string) => todo!(),
//             Value::Float(_) => todo!(),
//             Value::Integer(_) => todo!(),
//             Value::Boolean(_) => todo!(),
//             Value::List(list) => {
//                 let mut mapped_list = Vec::with_capacity(list.len());

//                 for value in list {
//                     let mut context = VariableMap::new();

//                     context.set_value("input", value.clone())?;

//                     let mapped_value = function.run_with_context(&mut context)?;

//                     mapped_list.push(mapped_value);
//                 }

//                 Ok(Value::List(mapped_list))
//             }
//             Value::Empty => todo!(),
//             Value::Map(_map) => todo!(),
//             Value::Table(_) => todo!(),
//             Value::Function(_) => todo!(),
//         }
//     }
// }
// pub struct Status;

// impl Macro for Status {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "git_status",
//             description: "Get the repository status for the current directory.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         argument.as_empty()?;

//         let repo = Repository::open(".")?;
//         let mut table = Table::new(vec![
//             "path".to_string(),
//             "status".to_string(),
//             "staged".to_string(),
//         ]);

//         for entry in repo.statuses(None)?.into_iter() {
//             let (status, staged) = {
//                 if entry.status().is_wt_new() {
//                     ("created".to_string(), false)
//                 } else if entry.status().is_wt_deleted() {
//                     ("deleted".to_string(), false)
//                 } else if entry.status().is_wt_modified() {
//                     ("modified".to_string(), false)
//                 } else if entry.status().is_index_new() {
//                     ("created".to_string(), true)
//                 } else if entry.status().is_index_deleted() {
//                     ("deleted".to_string(), true)
//                 } else if entry.status().is_index_modified() {
//                     ("modified".to_string(), true)
//                 } else if entry.status().is_ignored() {
//                     continue;
//                 } else {
//                     ("".to_string(), false)
//                 }
//             };
//             let path = entry.path().unwrap().to_string();

//             table.insert(vec![
//                 Value::String(path),
//                 Value::String(status),
//                 Value::Boolean(staged),
//             ])?;
//         }

//         Ok(Value::Table(table))
//     }
// }

// pub struct DocumentConvert;

// impl Macro for DocumentConvert {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "convert_document",
//             description: "Convert a file's contents to a format and set the extension.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let argument = argument.as_list()?;

//         if argument.len() != 3 {
//             return Err(Error::WrongFunctionArgumentAmount {
//                 expected: 3,
//                 actual: argument.len(),
//             });
//         }

//         let (path, from, to) = (
//             argument[0].as_string()?,
//             argument[1].as_string()?,
//             argument[2].as_string()?,
//         );
//         let mut file_name = PathBuf::from(&path);
//         file_name.set_extension(to);
//         let new_file_name = file_name.to_str().unwrap();
//         let script = format!("pandoc --from {from} --to {to} --output {new_file_name} {path}");

//         Command::new("fish").arg("-c").arg(script).spawn()?.wait()?;

//         Ok(Value::Empty)
//     }
// }

// pub struct Trash;

// impl Macro for Trash {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "trash_dir",
//             description: "Move a directory to the trash.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let path = argument.as_string()?;

//         trash::delete(path)?;

//         Ok(Value::Empty)
//     }
// }

// pub struct Get;

// impl Macro for Get {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "get",
//             description: "Extract a value from a collection.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let argument_list = argument.as_list()?;
//         let collection = &argument_list[0];
//         let index = &argument_list[1];

//         if let Ok(list) = collection.as_list() {
//             let index = index.as_int()?;
//             let value = list.get(index as usize).unwrap_or(&Value::Empty);

//             return Ok(value.clone());
//         }

//         if let Ok(table) = collection.as_table() {
//             let index = index.as_int()?;
//             let get_row = table.get(index as usize);

//             if let Some(row) = get_row {
//                 return Ok(Value::List(row.clone()));
//             }
//         }

//         Err(Error::TypeError {
//             expected: &[
//                 ValueType::List,
//                 ValueType::Map,
//                 ValueType::Table,
//                 ValueType::String,
//             ],
//             actual: collection.clone(),
//         })
//     }
// }

// pub struct ToCsv;

// impl Macro for ToCsv {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "to_csv",
//             description: "Convert a value to a string of comma-separated values.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let mut buffer = Vec::new();
//         let mut writer = csv::Writer::from_writer(&mut buffer);

//         match argument {
//             Value::String(string) => {
//                 writer.write_record([string])?;
//             }
//             Value::Float(float) => {
//                 writer.write_record(&[float.to_string()])?;
//             }
//             Value::Integer(integer) => {
//                 writer.write_record(&[integer.to_string()])?;
//             }
//             Value::Boolean(boolean) => {
//                 writer.write_record(&[boolean.to_string()])?;
//             }
//             Value::List(list) => {
//                 let string_list = list.iter().map(|value| value.to_string());

//                 writer.write_record(string_list)?;
//             }
//             Value::Empty => {}
//             Value::Map(map) => {
//                 writer.write_record(map.inner().keys())?;
//                 writer.write_record(map.inner().values().map(|value| value.to_string()))?;
//             }
//             Value::Table(table) => {
//                 writer.write_record(table.column_names())?;

//                 for row in table.rows() {
//                     let row_string = row.iter().map(|value| value.to_string());

//                     writer.write_record(row_string)?;
//                 }
//             }
//             Value::Function(_) => todo!(),
//         }

//         writer.flush()?;

//         Ok(Value::String(
//             String::from_utf8_lossy(writer.get_ref()).to_string(),
//         ))
//     }
// }

// pub struct FromJson;

// impl Macro for FromJson {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "from_json",
//             description: "Convert JSON to a whale value.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         if let Ok(string) = argument.as_string() {
//             let json: JsonValue = json::parse(string)?;
//             let value = Value::try_from(json)?;

//             Ok(value)
//         } else {
//             Err(Error::ExpectedString {
//                 actual: argument.clone(),
//             })
//         }
//     }
// }

// pub struct Count;

// impl Macro for Count {
//     fn info(&self) -> MacroInfo<'static> {
//         MacroInfo {
//             identifier: "count",
//             description: "Return the number of items in a value.",
//         }
//     }

//     fn run(&self, argument: &Value) -> Result<Value> {
//         let len = match argument {
//             Value::String(string) => string.len(),
//             Value::List(list) => list.len(),
//             Value::Map(map) => map.len(),
//             Value::Table(table) => table.len(),
//             Value::Function(_) | Value::Float(_) | Value::Integer(_) | Value::Boolean(_) => 1,
//             Value::Empty => 0,
//         };

//         Ok(Value::Integer(len as i64))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_formatting() {
        for function in MACRO_LIST {
            let identifier = function.info().identifier;

            assert_eq!(identifier.to_lowercase(), identifier);
            assert!(identifier.is_ascii());
            assert!(!identifier.is_empty());
            assert!(!identifier.contains(' '));
            assert!(!identifier.contains(':'));
            assert!(!identifier.contains('.'));
            assert!(!identifier.contains('-'));
        }
    }
}
