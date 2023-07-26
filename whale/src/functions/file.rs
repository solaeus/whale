use std::{
    fs::{copy, metadata, remove_file, OpenOptions},
    io::{Read as StdRead, Write as StdWrite},
    path::PathBuf,
    process::Command,
};

use crate::{BuiltinFunction, Error, FunctionInfo, Result, Value};

pub struct Convert;

impl BuiltinFunction for Convert {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::convert",
            description: "Convert a file's contents to a format and set the extension.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut argument = argument.as_tuple()?;

        if argument.len() != 3 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 3,
                actual: argument.len(),
            });
        }

        let (from, to, path) = (
            argument.pop().unwrap().as_string()?,
            argument.pop().unwrap().as_string()?,
            argument.pop().unwrap().as_string()?,
        );
        let mut file_name = PathBuf::from(&path);
        file_name.set_extension(&to);
        let new_file_name = file_name.to_str().unwrap();
        let script = format!("pandoc --from {from} --to {to} --output {new_file_name} {path}");

        Command::new("fish").arg("-c").arg(script).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}

pub struct Read;

impl BuiltinFunction for Read {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::read",
            description: "Read file contents.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let mut contents = String::new();

        OpenOptions::new()
            .read(true)
            .create(false)
            .open(&path)?
            .read_to_string(&mut contents)?;

        Ok(Value::String(contents))
    }
}

pub struct Write;

impl BuiltinFunction for Write {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::write",
            description: "Write data to a file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let strings = argument.as_tuple()?;

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
            let content = content.as_string()?;

            file.write_all(content.as_bytes())?;
        }

        Ok(Value::Empty)
    }
}

pub struct FileAppend;

impl BuiltinFunction for FileAppend {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::append",
            description: "Append data to a file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let strings = argument.as_tuple()?;

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

pub struct Remove;

impl BuiltinFunction for Remove {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::remove",
            description: "Remove files.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        remove_file(path)?;

        Ok(Value::Empty)
    }
}

pub struct Move;

impl BuiltinFunction for Move {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::move",
            description: "Move a file to a new location.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut paths = argument.as_tuple()?;

        if paths.len() != 2 {
            return Err(Error::WrongFunctionArgumentAmount {
                expected: 2,
                actual: paths.len(),
            });
        }

        let to = paths.pop().unwrap().as_string()?;
        let from = paths.pop().unwrap().as_string()?;

        copy(&from, to).and_then(|_| remove_file(from))?;

        Ok(Value::Empty)
    }
}

pub struct Metadata;

impl BuiltinFunction for Metadata {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::metadata",
            description: "Get meteadata for files.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let metadata = metadata(path)?;

        Ok(Value::String(format!("{:#?}", metadata)))
    }
}
