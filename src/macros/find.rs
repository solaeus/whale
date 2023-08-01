use crate::{Macro, MacroInfo, Result, Value};

pub struct Find;

impl Macro for Find {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "find",
            description: "Search for a value in a collection of values.",
        }
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}
