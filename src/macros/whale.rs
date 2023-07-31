use std::{fs, time::Instant};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{Function, Macro, MacroInfo, Result, Value, VariableMap};

pub struct Repeat;

impl Macro for Repeat {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "repeat",
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
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "run",
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
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "async",
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

        let start = Instant::now();
        let results = functions
            .par_iter()
            .map(|function| function.run())
            .map(|result| {
                let elapsed = Value::Integer(start.elapsed().as_millis() as i64);
                let mut map = VariableMap::new();

                match result {
                    Ok(value) => {
                        let _ = map.set_value("output", value);
                        let _ = map.set_value("time", elapsed);

                        Value::Map(map)
                    }
                    Err(error) => Value::String(error.to_string()),
                }
            })
            .collect();

        Ok(Value::List(results))
    }
}

pub struct Pipe;

impl Macro for Pipe {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "pipe",
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
