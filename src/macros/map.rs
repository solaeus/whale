use crate::{Macro, MacroInfo, Result, Value, VariableMap};

pub struct Map;

impl Macro for Map {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "map",
            description: "Change each value with a function.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let value = &argument[0];
        let function = argument[1].as_function()?;

        match value {
            Value::String(_string) => todo!(),
            Value::Float(_) => todo!(),
            Value::Integer(_) => todo!(),
            Value::Boolean(_) => todo!(),
            Value::List(list) => {
                let mut mapped_list = Vec::with_capacity(list.len());

                for value in list {
                    let mut context = VariableMap::new();

                    context.set_value("input", value.clone())?;

                    let mapped_value = function.run_with_context(&mut context)?;

                    mapped_list.push(mapped_value);
                }

                Ok(Value::List(mapped_list))
            }
            Value::Empty => todo!(),
            Value::Map(_map) => todo!(),
            Value::Table(_) => todo!(),
            Value::Function(_) => todo!(),
        }
    }
}
