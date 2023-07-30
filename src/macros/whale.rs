use std::fs;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Function, FunctionInfo, Macro, Result, Value};

pub struct Repeat;

impl Macro for Repeat {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::repeat",
            description: "Run a function the given number of times.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let function = argument[0].as_function()?;
        let count = argument[1].as_int()?;
        let mut result_list = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let result = function.run()?;

            result_list.push(result);
        }

        Ok(Value::List(result_list))
    }
}

pub struct Run;

impl Macro for Run {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::run",
            description: "Run a whale file.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let path = argument.as_string()?;
        let file_contents = fs::read_to_string(path)?;

        Function::new(file_contents).run()
    }
}

pub struct Async;

impl Macro for Async {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::async",
            description: "Run functions in parallel.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        let mut functions = Vec::new();

        for value in argument_list {
            let function = value.as_function()?;
            functions.push(function);
        }

        functions.par_iter().for_each(|function| {
            let _ = function.run();
        });

        Ok(Value::Empty)
    }
}

pub struct Pipe;

impl Macro for Pipe {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "whale::pipe",
            description: "Process a value with a list of functions.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        let input = &argument_list[0];
        let pipe = &argument_list[1..];
        let mut accumulator = input.clone();

        for value in pipe {
            accumulator = value.as_function()?.run()?;
        }

        Ok(accumulator)
    }
}
