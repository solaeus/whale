use crate::{Error, Macro, MacroInfo, Result, Value, ValueType};

pub struct If;

impl Macro for If {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo { identifier: "if", description: "Evaluates the first argument. If true, it does the second argument. If false, it does the third argument" }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_fixed_len_list(3)?;
        let (condition, if_true, if_false) = (&argument[0], &argument[1], &argument[2]);

        let condition_is_true = if let Ok(boolean) = condition.as_boolean() {
            boolean
        } else if let Ok(function) = condition.as_function() {
            function.run()?.as_boolean()?
        } else {
            return Err(Error::TypeError {
                expected: &[ValueType::Boolean, ValueType::Function],
                actual: condition.clone(),
            });
        };

        let should_yield = if condition_is_true { if_true } else { if_false };

        if let Ok(function) = should_yield.as_function() {
            function.run()
        } else {
            Ok(should_yield.clone())
        }
    }
}

pub struct While;

impl Macro for While {
    fn info(&self) -> MacroInfo<'static> {
        todo!()
    }

    fn run(&self, _argument: &Value) -> Result<Value> {
        todo!()
    }
}
