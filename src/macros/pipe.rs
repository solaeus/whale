use crate::{Macro, MacroInfo, Result, Value, VariableMap};

pub struct Pipe;

impl Macro for Pipe {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "pipe",
            description: "Execute a list of functions, passing the subsequent output to each.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let mut context = VariableMap::new();

        for value in argument {
            value.as_function()?.run_with_context(&mut context)?;
        }

        Ok(Value::Map(context))
    }
}
