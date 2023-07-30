use crate::{BuiltinFunction, FunctionInfo, Result, Value};

pub struct Output;

impl BuiltinFunction for Output {
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
