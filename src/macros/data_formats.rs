//! Convert values to and from data formats like JSON and TOML.

use crate::{Macro, MacroInfo, Result, Value};

pub struct FromJson;

impl Macro for FromJson {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "from_json",
            description: "Get a whale value from a JSON string.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;
        let value = serde_json::from_str(argument)?;

        Ok(value)
    }
}
