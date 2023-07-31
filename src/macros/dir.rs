use crate::{Error, Macro, MacroInfo, Result, Table, Value};

use std::{fs, path::PathBuf};

#[derive(Copy, Clone)]
pub struct Create;

impl Macro for Create {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "dir::create",
            description: "Create one or more directories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        fs::create_dir_all(path)?;

        Ok(Value::Empty)
    }
}

fn read_dir(path: &str) -> Result<Value> {
    let dir = fs::read_dir(path)?;
    let mut file_table = Table::new(vec![
        "path".to_string(),
        "modified".to_string(),
        "read only".to_string(),
        "size".to_string(),
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
        let modified = if let Ok(modified) = metadata.modified() {
            modified.elapsed().unwrap().as_secs()
        } else {
            u64::MAX
        };
        let read_only = format!("{:?}", metadata.permissions().readonly());
        let size = metadata.len();

        file_table.insert(vec![
            Value::String(file_name),
            Value::Integer(modified as i64),
            Value::String(read_only),
            Value::Integer(size as i64),
        ])?;
    }

    Ok(Value::Table(file_table))
}

pub struct Read;

impl Macro for Read {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "dir::read",
            description: "Read the content of a directory.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        read_dir(path)
    }
}

pub struct Remove;

impl Macro for Remove {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "dir::remove",
            description: "Remove directories.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        fs::remove_file(path)?;

        Ok(Value::Empty)
    }
}

pub struct Trash;

impl Macro for Trash {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "dir::trash",
            description: "Move a directory to the trash.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;

        trash::delete(&path)?;

        Ok(Value::Empty)
    }
}

pub struct Move;

impl Macro for Move {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "dir::move",
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
        let file_list = read_dir(current_path)?;

        for path in file_list.as_list()? {
            let path = PathBuf::from(path.as_string()?);
            let new_path = PathBuf::from(&target_path).join(&path);

            if path.is_file() {
                fs::copy(&path, &target_path)?;
            }

            if path.is_symlink() && path.symlink_metadata()?.is_file() {
                fs::copy(&path, new_path)?;
            }
        }

        Ok(Value::Empty)
    }
}

pub struct Copy;

impl Macro for Copy {
    fn info(&self) -> MacroInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}

pub struct Metadata;

impl Macro for Metadata {
    fn info(&self) -> MacroInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}
