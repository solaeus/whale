use crate::{BuiltinFunction, Error, FunctionInfo, Result, Table, Value};

use std::{fs, path::PathBuf};

#[derive(Copy, Clone)]
pub struct Create;

impl BuiltinFunction for Create {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
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

pub struct Read;

impl BuiltinFunction for Read {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "dir::read",
            description: "Read the content of a directory.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let dir = fs::read_dir(path)?;
        let mut file_table = Table::new(vec![
            "path".to_string(),
            "modified".to_string(),
            "permissions".to_string(),
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
            let modified = format!("{:?}", metadata.modified());
            let permisssions = format!("{:?}", metadata.permissions());

            file_table.insert(vec![
                Value::String(file_name),
                Value::String(modified),
                Value::String(permisssions),
            ])?;
        }

        Ok(Value::Table(file_table))
    }
}

pub struct Remove;

impl BuiltinFunction for Remove {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
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

impl BuiltinFunction for Trash {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
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

impl BuiltinFunction for Move {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "dir::move",
            description: "Move a directory to a new path.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut argument = argument.as_tuple()?;

        if argument.len() != 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: argument.len(),
            });
        }

        let target_path = argument.pop().unwrap().as_string()?;
        let current_path = argument.pop().unwrap();
        let file_list = Read.run(&current_path)?.as_tuple()?;

        for path in file_list {
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

impl BuiltinFunction for Copy {
    fn info(&self) -> FunctionInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}

pub struct Metadata;

impl BuiltinFunction for Metadata {
    fn info(&self) -> FunctionInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}
