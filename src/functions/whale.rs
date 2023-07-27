use std::fs;

use crate::{eval, BuiltinFunction, Error, FunctionInfo, Result, Value};

pub struct Repeat;

impl BuiltinFunction for Repeat {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::repeat",
            description: "Run a whale string the given number of times.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let input_string = argument[0].as_string()?;
        let count = argument[1].as_int()?;
        let mut result_list = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let result = eval(&input_string)?;

            result_list.push(result);
        }

        Ok(Value::List(result_list))
    }
}

pub struct Run;

impl BuiltinFunction for Run {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::run",
            description: "Run a whale string.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(input) = argument.as_string() {
            Ok(eval(&input)?)
        } else {
            Err(Error::ExpectedString {
                actual: argument.clone(),
            })
        }
    }
}

pub struct RunFile;

impl BuiltinFunction for RunFile {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::run_file",
            description: "Run one or more whale files.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        if let Ok(path) = argument.as_string() {
            let file_contents = fs::read_to_string(path)?;

            Ok(eval(&file_contents)?)
        } else if let Ok(paths) = argument.as_list() {
            let mut results = Vec::new();

            for path in paths {
                let path = path.as_string()?;
                let file_content = fs::read_to_string(path)?;
                let value = eval(&file_content)?;

                results.push(value);
            }

            Ok(Value::List(results))
        } else {
            Err(Error::ExpectedString {
                actual: argument.clone(),
            })
        }
    }
}

pub struct Async;

impl BuiltinFunction for Async {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::async",
            description: "Run whale files simultaneously.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;

        Ok(eval(&argument)?)
    }
}
