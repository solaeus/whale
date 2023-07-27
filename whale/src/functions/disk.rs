use std::process::Command;

use sysinfo::{DiskExt, System, SystemExt};

use crate::{BuiltinFunction, FunctionInfo, Result, Table, Value, VariableMap};

pub struct List;

impl BuiltinFunction for List {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "disk::list",
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
                Value::Int(total_space),
                Value::Int(available_space),
                Value::Boolean(is_removable),
            ];

            disk_table.insert(row)?;
        }

        Ok(Value::Table(disk_table))
    }
}

pub struct Partition;

impl BuiltinFunction for Partition {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "disk::partition",
            description: "Partition a disk, clearing its content.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_map()?;
        let path = argument
            .get_value("path")?
            .unwrap_or_default()
            .as_string()?;
        let label = argument
            .get_value("label")?
            .unwrap_or_default()
            .as_string()?;
        let name = argument
            .get_value("name")?
            .unwrap_or_default()
            .as_string()?;
        let filesystem = argument
            .get_value("filesystem")?
            .unwrap_or_default()
            .as_string()?;
        let mut range = argument
            .get_value("range")?
            .unwrap_or_default()
            .as_tuple()?;

        if range.len() != 2 {
            return Err(crate::Error::ExpectedFixedLenTuple {
                expected_len: 2,
                actual: Value::Tuple(range),
            });
        }

        let range_end = range.pop().unwrap_or_default().as_string()?;
        let range_start = range.pop().unwrap_or_default().as_string()?;

        let script = format!(
            "sudo parted {path} mklabel {label} mkpart {name} {filesystem} {range_start} {range_end}"
        );

        Command::new("fish")
            .arg("-c")
            .arg(&script)
            .spawn()?
            .wait()?;

        Ok(Value::String(path))
    }
}
