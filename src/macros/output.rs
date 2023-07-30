use crate::{FunctionInfo, Macro, Result, Value};

pub struct Output;

impl Macro for Output {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "output",
            description: "Print a value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        println!("{argument}");
        Ok(Value::Empty)
    }
}
