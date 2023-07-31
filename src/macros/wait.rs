use std::{path::PathBuf, thread::sleep, time::Duration};

use crate::{Macro, Result, Value};

pub struct Watch;

impl Macro for Watch {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "wait::watch",
            description: "Wait until a file changes.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;
        let path = PathBuf::from(argument);
        let modified_old = path.metadata()?.modified()?;
        let wait_time = loop {
            let modified_new = path.metadata()?.modified()?;

            if modified_old != modified_new {
                break modified_new
                    .duration_since(modified_old)
                    .unwrap_or_default()
                    .as_millis() as i64;
            }
        };

        Ok(Value::Integer(wait_time))
    }
}

pub struct Seconds;

impl Macro for Seconds {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "wait::seconds",
            description: "Wait for the given number of seconds.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_int()?;

        sleep(Duration::from_secs(argument as u64));

        Ok(Value::Empty)
    }
}
