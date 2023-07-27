use crate::{BuiltinFunction, FunctionInfo, Result, Table, Value};

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
        let mut argument = argument.as_list()?;
        let row = argument.pop().unwrap().as_list()?;
        let mut table = argument.pop().unwrap().as_table()?;

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
        let mut argument = argument.as_list()?;
        let expected = argument.pop().unwrap();
        let column_name = argument.pop().unwrap().as_string()?;
        let table = argument.pop().unwrap().as_table()?;
        let find = table.get_where(&column_name, &expected);

        if let Some(row) = find {
            Ok(Value::List(row.clone()))
        } else {
            Ok(Value::Empty)
        }
    }
}
