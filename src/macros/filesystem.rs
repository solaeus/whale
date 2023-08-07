//! Tools for files and directories.

use std::{
    fs::{self, OpenOptions},
    io::{Read, Write as IoWrite},
    path::PathBuf,
};

use crate::{Error, Macro, MacroInfo, Result, Table, Value, ValueType};

pub struct Append;

impl Macro for Append {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "append",
            description: "Append data to a file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let arguments = argument.as_fixed_len_list(2)?;
        let path = arguments[0].as_string()?;
        let content = arguments[1].as_string()?;
        let mut file = OpenOptions::new().append(true).open(path)?;

        file.write_all(content.as_bytes())?;

        Ok(Value::Empty)
    }
}

pub struct CreateDir;

impl Macro for CreateDir {
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
            return Err(Error::TypeError {
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

pub struct ReadFile;

impl Macro for ReadFile {
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

pub struct RemoveDir;

impl Macro for RemoveDir {
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

pub struct MoveDir;

impl Macro for MoveDir {
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

pub struct Trash;

impl Macro for Trash {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "trash",
            description: "Move a file or directory to the trash.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;

        trash::delete(path)?;

        Ok(Value::Empty)
    }
}

pub struct Write;

impl Macro for Write {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "write",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let path = PathBuf::from("./target/test_message.txt");
        let message = "hiya".to_string();

        let _ = std::fs::remove_file(&path);

        let path_value = Value::String(path.to_string_lossy().to_string());
        let message_value = Value::String(message.clone());

        Write
            .run(&Value::List(vec![path_value, message_value]))
            .unwrap();

        assert!(path.is_file());
    }

    #[test]
    fn append() {
        let path = PathBuf::from("./target/test_message.txt");
        let message = "hiya".to_string();

        let _ = std::fs::remove_file(&path);

        let path_value = Value::String(path.to_string_lossy().to_string());
        let message_value = Value::String(message.clone());

        Write
            .run(&Value::List(vec![
                path_value.clone(),
                message_value.clone(),
            ]))
            .unwrap();
        Append
            .run(&Value::List(vec![path_value, message_value]))
            .unwrap();

        let read = fs::read_to_string(&path).unwrap();

        assert_eq!("hiyahiya", read);
    }

    #[test]
    fn read_file() {
        let path = PathBuf::from("./target/test_message.txt");
        let message = "hiya".to_string();

        let _ = std::fs::remove_file(&path);

        let path_value = Value::String(path.to_string_lossy().to_string());
        let message_value = Value::String(message.clone());

        Write
            .run(&Value::List(vec![path_value.clone(), message_value]))
            .unwrap();

        let test = ReadFile.run(&path_value).unwrap();
        let read = fs::read_to_string(&path).unwrap();

        assert_eq!(test, Value::String(read));
    }
}
