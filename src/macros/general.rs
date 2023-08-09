use crate::{Macro, MacroInfo, Result, Value};

pub struct Output;

impl Macro for Output {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "output",
            description: "Print a value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        println!("{argument}");

        Ok(Value::Empty)
    }
}
