/// This module contains whale's built-in macro system. Every macro is listed
/// alphabetically. Use [call_macro] to check an identifier against every macro.
///
/// ## Writing macros
///
/// - snake case identifier, this is enforced by a test
/// - the type name should be the identifier in upper camel case
/// - always verify user input, this creates helpful errors
/// - the description should be brief, it will display in the shell
/// - maintain alphabetical order
///
/// ## Usage
///
/// Macros can be used in Rust by passing a Value to the run method.
///
/// ```
/// let value = Value::List(vec![1, 2,3]);
/// let count = Count.run(value).as_string().unwrap();
///
/// assert_eq!(count, 3);
/// ```
use std::{
    convert::{TryFrom, TryInto},
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use git2::Repository;
use json::JsonValue;
use rand::{random, thread_rng, Rng};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use sys_info::{cpu_num, cpu_speed, hostname};
use sysinfo::{DiskExt, System, SystemExt};

use crate::{
    error::expect_function_argument_length, Error, Function, Result, Table, Value, ValueType,
    VariableMap,
};

/// Master list of all macros.
///
/// This list is used to match identifiers with macros and to provide info to
/// the shell.
pub const MACRO_LIST: [&'static dyn Macro; 47] = [
    &Async,
    &Bash,
    &CoprRepositories,
    &Count,
    &Create,
    &DirCreate,
    &ReadDir,
    &Download,
    &FileAppend,
    &FileRead,
    &FileWrite,
    &Fish,
    &FromJson,
    &Get,
    &Insert,
    &Install,
    &ListDisks,
    &Transform,
    &FileMetadata,
    &Map,
    &MoveFile,
    &Output,
    &Partition,
    &Pipe,
    &RandomFloat,
    &RandomInteger,
    &RandomString,
    &Raw,
    &RemoveFile,
    &RemoveFile,
    &Repeat,
    &RpmRepositories,
    &Run,
    &Seconds,
    &Select,
    &Sh,
    &Sort,
    &Status,
    &SystemCpu,
    &SystemInfo,
    &ToCsv,
    &Trash,
    &Uninstall,
    &Upgrade,
    &Watch,
    &Where,
    &Zsh,
];

/// Internal whale function with its business logic and all information.
pub trait Macro: Sync + Send {
    fn info(&self) -> MacroInfo<'static>;
    fn run(&self, argument: &Value) -> Result<Value>;
}

/// Information needed for each function.
pub struct MacroInfo<'a> {
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

        Function::new(file_contents).run()
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
        let mut functions = Vec::new();

        for value in argument_list {
            let function = value.as_function()?;
            functions.push(function);
        }

        let start = Instant::now();
        let results = functions
            .par_iter()
            .map(|function| function.run())
            .map(|result| {
                let elapsed = Value::Integer(start.elapsed().as_millis() as i64);
                let mut map = VariableMap::new();

                match result {
                    Ok(value) => {
                        let _ = map.set_value("output", value);
                        let _ = map.set_value("time", elapsed);

                        Value::Map(map)
                    }
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

pub struct Create;

impl Macro for Create {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "create_table",
            description: "Define a new table with a list of column names and list of rows.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let column_names = argument[0]
            .as_list()?
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        let column_count = column_names.len();
        let rows = argument[1].as_list()?;
        let mut table = Table::new(column_names);

        for row in rows {
            let row = row.as_list()?.clone();

            expect_function_argument_length(row.len(), column_count)?;

            table.insert(row).unwrap();
        }

        Ok(Value::Table(table))
    }
}

pub struct Insert;

impl Macro for Insert {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "insert",
            description: "Add a new row to a table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let mut table = argument[0].as_table()?.clone();

        for row in &argument[1..] {
            let row = row.as_list()?.clone();

            table.insert(row)?;
        }

        Ok(Value::Table(table))
    }
}

pub struct Select;

impl Macro for Select {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "select",
            description: "Return a map with the selected columns.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_length(argument.len(), 2)?;

        let columns = argument[1].as_list()?;
        let map = argument[0].as_map()?;
        let mut column_names = Vec::new();

        for column in columns {
            let name = column.as_string()?;

            column_names.push(name.clone());
        }

        let mut selected = VariableMap::new();

        for (key, value) in map.inner() {
            if column_names.contains(key) {
                selected.set_value(key, value.clone())?;
            }
        }

        Ok(Value::Map(selected))
    }
}

pub struct ForEach;

impl Macro for ForEach {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "for_each",
            description: "Run an operation on every item in a collection.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_length(argument.len(), 2)?;

        let table = argument[0].as_table()?;
        let columns = argument[1].as_list()?;
        let mut column_names = Vec::new();

        for column in columns {
            let name = column.as_string()?;

            column_names.push(name.clone());
        }

        let selected = table.select(&column_names);

        Ok(Value::Table(selected))
    }
}

pub struct Where;

impl Macro for Where {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "where",
            description: "Keep rows matching a predicate.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        expect_function_argument_length(argument_list.len(), 2)?;

        let collection = &argument_list[0];
        let function = argument_list[1].as_function()?;

        if let Ok(list) = collection.as_list() {
            let mut context = VariableMap::new();
            let mut new_list = Vec::new();

            for value in list {
                context.set_value("input", value.clone())?;
                let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

                if keep_row {
                    new_list.push(value.clone());
                }
            }

            return Ok(Value::List(new_list));
        }

        if let Ok(table) = collection.as_table() {
            let mut context = VariableMap::new();
            let mut new_table = Table::new(table.column_names().clone());

            for row in table.rows() {
                for (column_index, cell) in row.iter().enumerate() {
                    let column_name = table.column_names().get(column_index).unwrap();

                    context.set_value(column_name, cell.clone())?;
                }
                let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

                if keep_row {
                    new_table.insert(row.clone())?;
                }
            }

            return Ok(Value::Table(new_table));
        }

        Err(Error::ExpectedValueType {
            expected: &[ValueType::List, ValueType::Table],
            actual: collection.clone(),
        })
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
            Err(crate::Error::ExpectedTuple {
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
#[derive(Copy, Clone)]
pub struct DirCreate;

impl Macro for DirCreate {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "create_dir",
            description: "Create one or more directories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        fs::create_dir_all(path)?;

        Ok(Value::Empty)
    }
}

pub struct ReadDir;

impl Macro for ReadDir {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "read_dir",
            description: "Read the content of a directory.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = if let Ok(path) = argument.as_string() {
            path
        } else if argument.is_empty() {
            "."
        } else {
            return Err(Error::ExpectedValueType {
                expected: &[ValueType::Empty, ValueType::String],
                actual: argument.clone(),
            });
        };
        let dir = fs::read_dir(path)?;
        let mut file_table = Table::new(vec![
            "path".to_string(),
            "size".to_string(),
            "created".to_string(),
            "accessed".to_string(),
            "modified".to_string(),
            "read only".to_string(),
        ]);

        for entry in dir {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = if file_type.is_dir() {
                let name = entry.file_name().into_string().unwrap_or_default();

                format!("{name}/")
            } else {
                entry.file_name().into_string().unwrap_or_default()
            };
            let metadata = entry.path().metadata()?;
            let created = metadata.accessed()?.elapsed()?.as_secs() / 60;
            let accessed = metadata.accessed()?.elapsed()?.as_secs() / 60;
            let modified = metadata.modified()?.elapsed()?.as_secs() / 60;
            let read_only = metadata.permissions().readonly();
            let size = metadata.len();

            file_table.insert(vec![
                Value::String(file_name),
                Value::Integer(size as i64),
                Value::Integer(created as i64),
                Value::Integer(accessed as i64),
                Value::Integer(modified as i64),
                Value::Boolean(read_only),
            ])?;
        }

        Ok(Value::Table(file_table))
    }
}

pub struct DirRemove;

impl Macro for DirRemove {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "remove_dir",
            description: "Remove directories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        fs::remove_file(path)?;

        Ok(Value::Empty)
    }
}

pub struct DirTrash;

impl Macro for DirTrash {
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

pub struct DirMove;

impl Macro for DirMove {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "move_dir",
            description: "Move a directory to a new path.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        if argument.len() != 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: argument.len(),
            });
        }

        let current_path = argument[0].as_string()?;
        let target_path = argument[1].as_string()?;
        let file_list = ReadDir.run(&Value::String(current_path.clone()))?;

        for path in file_list.as_list()? {
            let path = PathBuf::from(path.as_string()?);
            let new_path = PathBuf::from(&target_path).join(&path);

            if path.is_file() {
                fs::copy(&path, target_path)?;
            }

            if path.is_symlink() && path.symlink_metadata()?.is_file() {
                fs::copy(&path, new_path)?;
            }
        }

        Ok(Value::Empty)
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

pub struct FileRead;

impl Macro for FileRead {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "read_file",
            description: "Read file contents.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let mut contents = String::new();

        OpenOptions::new()
            .read(true)
            .create(false)
            .open(path)?
            .read_to_string(&mut contents)?;

        Ok(Value::String(contents))
    }
}

pub struct FileWrite;

impl Macro for FileWrite {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "write_file",
            description: "Write data to a file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let strings = argument.as_list()?;

        if strings.len() < 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: strings.len(),
            });
        }

        let path = strings.first().unwrap().as_string()?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        for content in &strings[1..] {
            let content = content.to_string();

            file.write_all(content.as_bytes())?;
        }

        Ok(Value::Empty)
    }
}

pub struct FileAppend;

impl Macro for FileAppend {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "append_file",
            description: "Append data to a file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let strings = argument.as_list()?;

        if strings.len() < 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: strings.len(),
            });
        }

        let path = strings.first().unwrap().as_string()?;
        let mut file = std::fs::OpenOptions::new().append(true).open(path)?;

        for content in &strings[1..] {
            let content = content.as_string()?;

            file.write_all(content.as_bytes())?;
        }

        Ok(Value::Empty)
    }
}

pub struct RemoveFile;

impl Macro for RemoveFile {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "remove_file",
            description: "Permanently remove one or more files.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        fs::remove_file(path)?;

        Ok(Value::Empty)
    }
}

pub struct MoveFile;

impl Macro for MoveFile {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "move_file",
            description: "Move a file to a new location.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let paths = argument.as_list()?;

        if paths.len() != 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: paths.len(),
            });
        }

        let from = paths[0].as_string()?;
        let to = paths[1].as_string()?;

        fs::copy(from, to).and_then(|_| fs::remove_file(from))?;

        Ok(Value::Empty)
    }
}

pub struct FileMetadata;

impl Macro for FileMetadata {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "file_metadata",
            description: "Get meteadata for files.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let metadata = fs::metadata(path)?;

        Ok(Value::String(format!("{:#?}", metadata)))
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
            return Err(crate::Error::ExpectedFixedLenTuple {
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

        Err(Error::ExpectedValueType {
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
