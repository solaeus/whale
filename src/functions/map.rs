use crate::{BuiltinFunction, FunctionInfo, Result, Value, VariableMap};

pub struct Map;

impl BuiltinFunction for Map {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "map",
            description: "Create a map from a list of key-value pairs.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let mut map = VariableMap::new();

        for pair in &argument {
            let pair = pair.as_list()?;

            println!("{pair:?}");

            if pair.len() != 2 {
                return Err(crate::Error::ExpectedFixedLenTuple {
                    expected_len: 2,
                    actual: Value::List(pair),
                });
            }

            let key = pair[0].as_string()?;
            let value = pair[1].clone();

            map.set_value(&key, value)?;
        }

        Ok(Value::Map(map))
    }
}
