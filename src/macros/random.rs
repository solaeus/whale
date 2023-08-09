use std::convert::TryInto;

use rand::{random, thread_rng, Rng};

use crate::{error::expect_function_argument_length, Error, Macro, MacroInfo, Result, Value};

pub struct RandomBoolean;

impl Macro for RandomBoolean {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_boolean",
            description: "Create a random boolean.",
            group: "random",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        let boolean = rand::thread_rng().gen();

        Ok(Value::Boolean(boolean))
    }
}

pub struct RandomInteger;

impl Macro for RandomInteger {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_integer",
            description: "Create a random integer.",
            group: "random",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Integer(max) => {
                let integer = rand::thread_rng().gen_range(0..*max);

                Ok(Value::Integer(integer))
            }
            Value::List(min_max) => {
                expect_function_argument_length(
                    self.info().identifier.to_string(),
                    min_max.len(),
                    2,
                )?;

                let min = min_max.get(0).unwrap().as_int()?;
                let max = min_max.get(1).unwrap().as_int()? + 1;
                let integer = rand::thread_rng().gen_range(min..max);

                Ok(Value::Integer(integer))
            }
            Value::Empty => Ok(crate::Value::Integer(random())),
            _ => todo!(),
        }
    }
}

pub struct RandomString;

impl Macro for RandomString {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_string",
            description: "Generate a random string.",
            group: "random",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Integer(length) => {
                let length: usize = length.unsigned_abs().try_into().unwrap_or(0);
                let mut random = String::with_capacity(length);

                for _ in 0..length {
                    let random_char = thread_rng().gen_range('A'..='z').to_string();

                    random.push_str(&random_char);
                }

                Ok(Value::String(random))
            }
            Value::Empty => {
                let mut random = String::with_capacity(10);

                for _ in 0..10 {
                    let random_char = thread_rng().gen_range('A'..='z').to_string();

                    random.push_str(&random_char);
                }

                Ok(Value::String(random))
            }
            _ => Err(Error::ExpectedEmpty {
                actual: argument.clone(),
            }),
        }
    }
}

pub struct RandomFloat;

impl Macro for RandomFloat {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "random_float",
            description: "Generate a random floating point value between 0 and 1.",
            group: "random",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        Ok(Value::Float(random()))
    }
}
