use crate::{error::expect_function_argument_amount, Macro, MacroInfo, Result, Table, Value};

pub struct Create;

impl Macro for Create {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::create",
            description: "Define a new table with a list of column names and list of rows.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let column_names = argument[0]
            .as_list()?
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        let column_count = column_names.len();
        let rows = argument[1].as_list()?;
        let mut table = Table::new(column_names);

        for row in rows {
            let row = row.as_list()?.clone();

            expect_function_argument_amount(row.len(), column_count)?;

            table.insert(row).unwrap();
        }

        Ok(Value::Table(table))
    }
}

pub struct Insert;

impl Macro for Insert {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::insert",
            description: "Add a new row to a table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let mut table = argument[0].as_table()?;

        for row in &argument[1..] {
            let row = row.as_list()?.clone();

            table.insert(row)?;
        }

        Ok(Value::Table(table))
    }
}

pub struct Find;

impl Macro for Find {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::find",
            description: "Return the first row that matches a predicate.",
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
            let mut new_table = Table::new(table.column_names().clone());
            new_table.insert(row.clone()).unwrap();

            Ok(Value::Table(new_table))
        } else {
            Ok(Value::Empty)
        }
    }
}

pub struct Filter;

impl Macro for Filter {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::filter",
            description: "Keep rows matching a predicate.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_amount(argument.len(), 3)?;

        let table = argument[0].as_table()?;
        let column_name = argument[1].as_string()?;
        let expected = &argument[2];
        let filtered_table = table.filter(column_name, expected);

        if let Some(table) = filtered_table {
            Ok(Value::Table(table))
        } else {
            Ok(Value::Empty)
        }
    }
}
