use std::convert::TryInto;

use rand::{random, thread_rng, Rng};

use crate::{BuiltinFunction, Error, FunctionInfo, Result, Value};

pub struct RandomInteger;

impl BuiltinFunction for RandomInteger {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "random::integer",
            description: "Create a random integer.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Int(max) => {
                let integer = rand::thread_rng().gen_range(0..*max);

                Ok(Value::Int(integer))
            }
            Value::Tuple(min_max) => {
                if min_max.len() != 2 {
                    return Err(Error::WrongFunctionArgumentAmount {
                        expected: 2,
                        actual: min_max.len(),
                    });
                }

                let min = min_max.get(0).unwrap().as_int()?;
                let max = min_max.get(1).unwrap().as_int()? + 1;
                let integer = rand::thread_rng().gen_range(min..max);

                Ok(Value::Int(integer))
            }
            Value::Empty => Ok(crate::Value::Int(random())),
            _ => todo!(),
        }
    }
}

pub struct RandomString;

impl BuiltinFunction for RandomString {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "random::string",
            description: "Generate a random string.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        match argument {
            Value::Int(length) => {
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

impl BuiltinFunction for RandomFloat {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "random::float",
            description: "Generate a random floating point value between 0 and 1.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        Ok(Value::Float(random()))
    }
}
