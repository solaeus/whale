use std::{
    fs::{copy, metadata, remove_file, OpenOptions},
    io::{Read as StdRead, Write as StdWrite},
    path::PathBuf,
    process::Command,
};

use crate::{Error, FunctionInfo, Macro, Result, Value};

pub struct Convert;

impl Macro for Convert {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::convert",
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
        file_name.set_extension(&to);
        let new_file_name = file_name.to_str().unwrap();
        let script = format!("pandoc --from {from} --to {to} --output {new_file_name} {path}");

        Command::new("fish").arg("-c").arg(script).spawn()?.wait()?;

        Ok(Value::Empty)
    }
}

pub struct Read;

impl Macro for Read {
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

impl Macro for Write {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::write",
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
            let content = content.as_string()?;

            file.write_all(content.as_bytes())?;
        }

        Ok(Value::Empty)
    }
}

pub struct FileAppend;

impl Macro for FileAppend {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::append",
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

pub struct Remove;

impl Macro for Remove {
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

impl Macro for Move {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "file::move",
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

        copy(&from, to).and_then(|_| remove_file(from))?;

        Ok(Value::Empty)
    }
}

pub struct Metadata;

impl Macro for Metadata {
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
