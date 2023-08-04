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
use std::{
    convert::{TryFrom, TryInto},
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use git2::Repository;
use json::JsonValue;
use rand::{random, thread_rng, Rng};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use sys_info::{cpu_num, cpu_speed, hostname};
use sysinfo::{DiskExt, System, SystemExt};

use crate::{Error, Function, Result, Table, Value, ValueType, VariableMap};

mod collections;
mod filesystem;
mod test;
mod values;

/// Master list of all macros.
///
/// This list is used to match identifiers with macros and to provide info to
/// the shell.
pub const MACRO_LIST: [&'static dyn Macro; 15] = [
    &filesystem::Append,
    &filesystem::CreateDir,
    &filesystem::FileMetadata,
    &filesystem::MoveDir,
    &filesystem::ReadDir,
    &filesystem::ReadFile,
    &filesystem::RemoveDir,
    &filesystem::Trash,
    &filesystem::Write,
    &collections::CreateTable,
    &collections::Insert,
    &collections::Where,
    &collections::Select,
    &test::Assert,
    &test::AssertEqual,
];

/// Internal whale function with its business logic and all information.
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
}

/// Searches all macros for a matching identifier and runs the corresponding
/// macro with the given input. Returns the function's output or an error.
///
/// The word "macro" is reserved in Rust, `r#macro` is the way to escape the
/// reserved keyword.
pub fn call_macro(identifier: &str, argument: &Value) -> Result<Value> {
    for r#macro in MACRO_LIST {
        if identifier == r#macro.info().identifier {
            return r#macro.run(argument);
        }
    }

    Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
}

pub struct Repeat;

impl Macro for Repeat {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "repeat",
            description: "Run a function the given number of times.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let function = argument[0].as_function()?;
        let count = argument[1].as_int()?;
        let mut result_list = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let result = function.run()?;

            result_list.push(result);
        }

        Ok(Value::List(result_list))
    }
}

pub struct Run;

impl Macro for Run {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "run",
            description: "Run a whale file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let file_contents = fs::read_to_string(path)?;

        Function::new(&file_contents).run()
    }
}

pub struct Async;

impl Macro for Async {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "async",
            description: "Run functions in parallel.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        let results = argument_list
            .par_iter()
            .map(|value| {
                let function = if let Ok(function) = value.as_function() {
                    function
                } else {
                    return value.clone();
                };

                match function.run() {
                    Ok(value) => value,
                    Err(error) => Value::String(error.to_string()),
                }
            })
            .collect();

        Ok(Value::List(results))
    }
}

pub struct Pipe;

impl Macro for Pipe {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "pipe",
            description: "Process a value with a list of functions.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        let input = &argument_list[0];
        let pipe = &argument_list[1..];
        let mut accumulator = input.clone();

        for value in pipe {
            accumulator = value.as_function()?.run()?;
        }

        Ok(accumulator)
    }
}

pub struct Watch;

impl Macro for Watch {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "watch",
            description: "Wait until a file changes.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;
        let path = PathBuf::from(argument);
        let modified_old = path.metadata()?.modified()?;
        let wait_time = loop {
            let modified_new = path.metadata()?.modified()?;

            if modified_old != modified_new {
                break modified_new
                    .duration_since(modified_old)
                    .unwrap_or_default()
                    .as_millis() as i64;
            }
        };

        Ok(Value::Integer(wait_time))
    }
}

pub struct Seconds;

impl Macro for Seconds {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "wait",
            description: "Wait for the given number of seconds.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_int()?;

        sleep(Duration::from_secs(argument as u64));

        Ok(Value::Empty)
    }
}

pub struct SystemInfo;

impl Macro for SystemInfo {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "system_info",
            description: "Get information on the system.",
        }
    }

    fn run(&self, argument: &Value) -> crate::Result<Value> {
        argument.as_empty()?;

        let mut map = VariableMap::new();

        map.set_value("hostname", Value::String(hostname()?))?;

        Ok(Value::Map(map))
    }
}

pub struct SystemCpu;

impl Macro for SystemCpu {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "system_cpu",
            description: "Get information on the system's CPU.",
        }
    }

    fn run(&self, argument: &Value) -> crate::Result<Value> {
        argument.as_empty()?;

        let mut table = Table::new(vec!["count".to_string(), "speed".to_string()]);
        let count = cpu_num().unwrap_or_default() as i64;
        let speed = cpu_speed().unwrap_or_default() as i64;

        table.insert(vec![Value::Integer(count), Value::Integer(speed)])?;

        Ok(Value::Table(table))
    }
}

pub struct Sort;

impl Macro for Sort {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "sort",
            description: "Apply default ordering.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(mut list) = argument.as_list().cloned() {
            list.sort();

            Ok(Value::List(list))
        } else if let Ok(map) = argument.as_map().cloned() {
            Ok(Value::Map(map))
        } else if let Ok(mut table) = argument.as_table().cloned() {
            table.sort();

            Ok(Value::Table(table))
        } else {
            Err(crate::Error::ExpectedList {
                actual: argument.clone(),
            })
        }
    }
}

pub struct RandomInteger;

impl Macro for RandomInteger {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_integer",
            description: "Create a random integer.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Integer(max) => {
                let integer = rand::thread_rng().gen_range(0..*max);

                Ok(Value::Integer(integer))
            }
            Value::List(min_max) => {
                if min_max.len() != 2 {
                    return Err(Error::WrongFunctionArgumentAmount {
                        expected: 2,
                        actual: min_max.len(),
                    });
                }

                let min = min_max.get(0).unwrap().as_int()?;
                let max = min_max.get(1).unwrap().as_int()? + 1;
                let integer = rand::thread_rng().gen_range(min..max);

                Ok(Value::Integer(integer))
            }
            Value::Empty => Ok(crate::Value::Integer(random())),
            _ => todo!(),
        }
    }
}

pub struct RandomString;

impl Macro for RandomString {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_string",
            description: "Generate a random string.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Integer(length) => {
                let length: usize = length.unsigned_abs().try_into().unwrap_or(0);
                let mut random = String::with_capacity(length);

                for _ in 0..length {
                    let random_char = thread_rng().gen_range('A'..='z').to_string();

                    random.push_str(&random_char);
                }

                Ok(Value::String(random))
            }
            Value::Empty => {
                let mut random = String::with_capacity(10);

                for _ in 0..10 {
                    let random_char = thread_rng().gen_range('A'..='z').to_string();

                    random.push_str(&random_char);
                }

                Ok(Value::String(random))
            }
            _ => Err(Error::ExpectedEmpty {
                actual: argument.clone(),
            }),
        }
    }
}

pub struct RandomFloat;

impl Macro for RandomFloat {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_float",
            description: "Generate a random floating point value between 0 and 1.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        Ok(Value::Float(random()))
    }
}

pub struct CoprRepositories;

impl Macro for CoprRepositories {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "enable_copr_repository",
            description: "Enable one or more COPR repositories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let repo_list_string = if let Ok(repo) = argument.as_string().cloned() {
            repo
        } else if let Ok(repos) = argument.as_list() {
            repos.iter().map(|value| value.to_string() + " ").collect()
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

impl Macro for Install {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "install_package",
            description: "Install one or more packages.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let package_list_string = if let Ok(package) = argument.as_string().cloned() {
            package
        } else if let Ok(packages) = argument.as_list() {
            packages
                .iter()
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

impl Macro for RpmRepositories {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "enable_rpm_repositories",
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
        } else if let Ok(repos) = argument.as_list() {
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

impl Macro for Uninstall {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "uninstall_package",
            description: "Uninstall one or more packages.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let package_list_string = if let Ok(package) = argument.as_string().cloned() {
            package
        } else if let Ok(packages) = argument.as_list() {
            packages
                .iter()
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

impl Macro for Upgrade {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "upgrade_packages",
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
pub struct Output;

impl Macro for Output {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "output",
            description: "Print a value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        println!("{argument}");
        Ok(Value::Empty)
    }
}
pub struct Download;

impl Macro for Download {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "download",
            description: "Download a file from a URL.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let url = argument.as_string()?;
        let script = format!("curl --tlsv1.2 -sSf {url}");
        let download = Command::new("fish")
            .arg("-c")
            .arg(script)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
            .stdout;

        Ok(Value::String(
            String::from_utf8_lossy(&download).to_string(),
        ))
    }
}

pub struct Map;

impl Macro for Map {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "map",
            description: "Create a map from a value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::String(_) => todo!(),
            Value::Float(_) => todo!(),
            Value::Integer(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::List(_) => todo!(),
            Value::Map(_) => todo!(),
            Value::Table(table) => Ok(Value::Map(VariableMap::from(table))),
            Value::Function(_) => todo!(),
            Value::Empty => todo!(),
        }
    }
}

pub struct Transform;

impl Macro for Transform {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "transform",
            description: "Change each value with a function.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let value = &argument[0];
        let function = argument[1].as_function()?;

        match value {
            Value::String(_string) => todo!(),
            Value::Float(_) => todo!(),
            Value::Integer(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::List(list) => {
                let mut mapped_list = Vec::with_capacity(list.len());

                for value in list {
                    let mut context = VariableMap::new();

                    context.set_value("input", value.clone())?;

                    let mapped_value = function.run_with_context(&mut context)?;

                    mapped_list.push(mapped_value);
                }

                Ok(Value::List(mapped_list))
            }
            Value::Empty => todo!(),
            Value::Map(_map) => todo!(),
            Value::Table(_) => todo!(),
            Value::Function(_) => todo!(),
        }
    }
}
pub struct Status;

impl Macro for Status {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "git_status",
            description: "Get the repository status for the current directory.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        let repo = Repository::open(".")?;
        let mut table = Table::new(vec![
            "path".to_string(),
            "status".to_string(),
            "staged".to_string(),
        ]);

        for entry in repo.statuses(None)?.into_iter() {
            let (status, staged) = {
                if entry.status().is_wt_new() {
                    ("created".to_string(), false)
                } else if entry.status().is_wt_deleted() {
                    ("deleted".to_string(), false)
                } else if entry.status().is_wt_modified() {
                    ("modified".to_string(), false)
                } else if entry.status().is_index_new() {
                    ("created".to_string(), true)
                } else if entry.status().is_index_deleted() {
                    ("deleted".to_string(), true)
                } else if entry.status().is_index_modified() {
                    ("modified".to_string(), true)
                } else if entry.status().is_ignored() {
                    continue;
                } else {
                    ("".to_string(), false)
                }
            };
            let path = entry.path().unwrap().to_string();

            table.insert(vec![
                Value::String(path),
                Value::String(status),
                Value::Boolean(staged),
            ])?;
        }

        Ok(Value::Table(table))
    }
}

pub struct DocumentConvert;

impl Macro for DocumentConvert {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "convert_document",
            description: "Convert a file's contents to a format and set the extension.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        if argument.len() != 3 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 3,
                actual: argument.len(),
            });
        }

        let (path, from, to) = (
            argument[0].as_string()?,
            argument[1].as_string()?,
            argument[2].as_string()?,
        );
        let mut file_name = PathBuf::from(&path);
        file_name.set_extension(to);
        let new_file_name = file_name.to_str().unwrap();
        let script = format!("pandoc --from {from} --to {to} --output {new_file_name} {path}");

        Command::new("fish").arg("-c").arg(script).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}

pub struct ListDisks;

impl Macro for ListDisks {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "list_disks",
            description: "List all block devices.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        let mut sys = System::new_all();
        sys.refresh_all();

        let mut disk_table = Table::new(vec![
            "name".to_string(),
            "kind".to_string(),
            "file system".to_string(),
            "mount point".to_string(),
            "total space".to_string(),
            "available space".to_string(),
            "is removable".to_string(),
        ]);

        for disk in sys.disks() {
            let name = disk.name().to_string_lossy().to_string();
            let kind = disk.kind();
            let file_system = String::from_utf8_lossy(disk.file_system()).to_string();
            let mount_point = disk.mount_point().to_str().unwrap().to_string();
            let total_space = disk.total_space() as i64;
            let available_space = disk.available_space() as i64;
            let is_removable = disk.is_removable();

            let row = vec![
                Value::String(name),
                Value::String(format!("{kind:?}")),
                Value::String(file_system),
                Value::String(mount_point),
                Value::Integer(total_space),
                Value::Integer(available_space),
                Value::Boolean(is_removable),
            ];

            disk_table.insert(row)?;
        }

        Ok(Value::Table(disk_table))
    }
}

pub struct Partition;

impl Macro for Partition {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "partition_disk",
            description: "Partition a disk, clearing its content.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_map()?;
        let path = argument
            .get_value("path")?
            .unwrap_or(Value::Empty)
            .as_string()?
            .clone();
        let label = argument
            .get_value("label")?
            .unwrap_or(Value::Empty)
            .as_string()?
            .clone();
        let name = argument
            .get_value("name")?
            .unwrap_or(Value::Empty)
            .as_string()?
            .clone();
        let filesystem = argument
            .get_value("filesystem")?
            .unwrap_or(Value::Empty)
            .as_string()?
            .clone();
        let range = argument
            .get_value("range")?
            .unwrap_or(Value::Empty)
            .as_list()?
            .clone();

        if range.len() != 2 {
            return Err(crate::Error::ExpectedFixedLenList {
                expected_len: 2,
                actual: Value::List(range),
            });
        }

        let range_start = range[0].as_string()?;
        let range_end = range[1].as_string()?;

        let script = format!(
            "sudo parted {path} mklabel {label} mkpart {name} {filesystem} {range_start} {range_end}"
        );

        Command::new("fish")
            .arg("-c")
            .arg(&script)
            .spawn()?
            .wait()?;

        Ok(Value::Empty)
    }
}

pub struct Trash;

impl Macro for Trash {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "trash_dir",
            description: "Move a directory to the trash.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;

        trash::delete(path)?;

        Ok(Value::Empty)
    }
}

pub struct Get;

impl Macro for Get {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "get",
            description: "Extract a value from a collection.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        let collection = &argument_list[0];
        let index = &argument_list[1];

        if let Ok(list) = collection.as_list() {
            let index = index.as_int()?;
            let value = list.get(index as usize).unwrap_or(&Value::Empty);

            return Ok(value.clone());
        }

        if let Ok(table) = collection.as_table() {
            let index = index.as_int()?;
            let get_row = table.get(index as usize);

            if let Some(row) = get_row {
                return Ok(Value::List(row.clone()));
            }
        }

        Err(Error::TypeError {
            expected: &[
                ValueType::List,
                ValueType::Map,
                ValueType::Table,
                ValueType::String,
            ],
            actual: collection.clone(),
        })
    }
}

pub struct ToCsv;

impl Macro for ToCsv {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "to_csv",
            description: "Convert a value to a string of comma-separated values.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut buffer = Vec::new();
        let mut writer = csv::Writer::from_writer(&mut buffer);

        match argument {
            Value::String(string) => {
                writer.write_record([string])?;
            }
            Value::Float(float) => {
                writer.write_record(&[float.to_string()])?;
            }
            Value::Integer(integer) => {
                writer.write_record(&[integer.to_string()])?;
            }
            Value::Boolean(boolean) => {
                writer.write_record(&[boolean.to_string()])?;
            }
            Value::List(list) => {
                let string_list = list.iter().map(|value| value.to_string());

                writer.write_record(string_list)?;
            }
            Value::Empty => {}
            Value::Map(map) => {
                writer.write_record(map.inner().keys())?;
                writer.write_record(map.inner().values().map(|value| value.to_string()))?;
            }
            Value::Table(table) => {
                writer.write_record(table.column_names())?;

                for row in table.rows() {
                    let row_string = row.iter().map(|value| value.to_string());

                    writer.write_record(row_string)?;
                }
            }
            Value::Function(_) => todo!(),
        }

        writer.flush()?;

        Ok(Value::String(
            String::from_utf8_lossy(writer.get_ref()).to_string(),
        ))
    }
}

pub struct FromJson;

impl Macro for FromJson {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "from_json",
            description: "Convert JSON to a whale value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(string) = argument.as_string() {
            let json: JsonValue = json::parse(string)?;
            let value = Value::try_from(json)?;

            Ok(value)
        } else {
            Err(Error::ExpectedString {
                actual: argument.clone(),
            })
        }
    }
}

pub struct Count;

impl Macro for Count {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "count",
            description: "Return the number of items in a value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let len = match argument {
            Value::String(string) => string.len(),
            Value::List(list) => list.len(),
            Value::Map(map) => map.len(),
            Value::Table(table) => table.len(),
            Value::Function(_) | Value::Float(_) | Value::Integer(_) | Value::Boolean(_) => 1,
            Value::Empty => 0,
        };

        Ok(Value::Integer(len as i64))
    }
}

pub struct Sh;

impl Macro for Sh {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "sh",
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

impl Macro for Bash {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "bash",
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

impl Macro for Fish {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "fish",
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

impl Macro for Zsh {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "zsh",
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

impl Macro for Raw {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "raw",
            description: "Run input as a command without a shell",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Command::new(argument).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}

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
