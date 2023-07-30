use sys_info::{self, cpu_num, cpu_speed, hostname};

use crate::{FunctionInfo, Macro, Table, Value, VariableMap};

pub struct SystemInfo;

impl Macro for SystemInfo {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "system::info",
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
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "system::cpu",
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
