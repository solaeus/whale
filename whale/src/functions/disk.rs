use std::process::Command;

use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

use crate::{BuiltinFunction, FunctionInfo, Result, Value, VariableMap};

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

        let mut disk_list = Vec::new();

        for disk in sys.disks() {
            let mut map = VariableMap::new(Some("disk::list.output".to_string()));
            let kind = disk.kind();
            map.set_value("kind", Value::String(format!("{kind:?}")))?;

            let name = disk.name();
            map.set_value("name", Value::String(format!("{name:?}")))?;

            let file_system = String::from_utf8_lossy(disk.file_system()).to_string();
            map.set_value("file_system", Value::String(file_system))?;

            let mount_point = disk.mount_point().to_str().unwrap().to_string();
            map.set_value("mount_point", Value::String(mount_point))?;

            let total_space = disk.total_space() as i64;
            map.set_value("total_space", Value::Int(total_space))?;

            let available_space = disk.available_space() as i64;
            map.set_value("available_space", Value::Int(available_space))?;

            map.set_value("is_removable", Value::Boolean(disk.is_removable()))?;

            disk_list.push(Value::Map(map));
        }

        Ok(Value::Tuple(disk_list))
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
