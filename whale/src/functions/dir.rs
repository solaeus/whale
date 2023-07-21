use crate::{BuiltinFunction, FunctionInfo, Result, Value};

use std::fs::{create_dir_all, read_dir, remove_file};

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
        create_dir_all(path)?;

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
        let dir = read_dir(path)?;
        let mut file_list = Vec::new();

        for entry in dir {
            let entry = entry?;
            let file_type = entry.file_type()?;

            let file_name = if file_type.is_dir() {
                let name = entry.file_name().into_string().unwrap_or_default();
                format!("{name}/")
            } else {
                entry.file_name().into_string().unwrap_or_default()
            };

            file_list.push(Value::String(file_name));
        }

        Ok(Value::Tuple(file_list))
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
        remove_file(path)?;

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
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
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

pub struct Write;

impl BuiltinFunction for Write {
    fn info(&self) -> FunctionInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}
