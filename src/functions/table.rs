use crate::{
    error::expect_function_argument_amount, BuiltinFunction, FunctionInfo, Result, Table, Value,
};

pub struct Create;

impl BuiltinFunction for Create {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "table::create",
            description: "Define a new table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        Ok(Value::Table(Table::from(argument.clone())))
    }
}

pub struct Insert;

impl BuiltinFunction for Insert {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "table::insert",
            description: "Add a new row to a table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_amount(argument.len(), 2)?;

        let mut table = argument[0].as_table()?;
        let row = argument[1].as_list()?;

        table.insert(row)?;

        Ok(Value::Table(table))
    }
}

pub struct Find;

impl BuiltinFunction for Find {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "table::find",
            description: "Search for a row based on a predicate.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_amount(argument.len(), 3)?;

        let table = argument[0].as_table()?;
        let column_name = argument[1].as_string()?;
        let expected = &argument[2];
        let find = table.get_where(&column_name, expected);

        if let Some(row) = find {
            Ok(Value::List(row.clone()))
        } else {
            Ok(Value::Empty)
        }
    }
}
