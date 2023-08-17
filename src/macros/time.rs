use chrono::Utc;

use crate::{Macro, MacroInfo, Result, Time, Value};

pub struct Now;

impl Macro for Now {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "now",
            description: "Return the current time.",
            group: "time",
        }
    }

    fn run(&self, argument: &crate::Value) -> Result<Value> {
        argument.as_empty()?;

        let time = Time::new(Utc::now(), Some(0));

        Ok(Value::Time(time))
    }
}

pub struct Local;

impl Macro for Local {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "local",
            description: "Show a time value adjusted for the current time zone.",
            group: "time",
        }
    }

    fn run(&self, argument: &crate::Value) -> Result<Value> {
        let argument = argument.as_time()?;

        Ok(Value::String(argument.local()))
    }
}
