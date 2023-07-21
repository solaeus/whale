use sys_info;

use crate::{BuiltinFunction, FunctionInfo, Value};

pub struct SystemInfo;

// impl BuiltinFunction for SystemInfo {
//     const FUNCTION_INFO: crate::FunctionInfo<'static> = FunctionInfo {
//         identifier: "system::info",
//         description: "Get all system information.",
//     };

//     fn run(_argument: &Value) -> crate::Result<Value> {
//         let info = PlatformInfo::new().unwrap();

//         Ok(Value::String(format!("{:?}", info)))
//     }
// }

pub struct SystemCpu;

impl BuiltinFunction for SystemCpu {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "system::cpu",
            description: "Get information on the system's CPU.",
        }
    }

    fn run(&self, argument: &Value) -> crate::Result<Value> {
        argument.as_empty()?;
        let speed = sys_info::cpu_speed().unwrap_or_default();
        Ok(Value::String(format!("{speed}")))
    }
}
