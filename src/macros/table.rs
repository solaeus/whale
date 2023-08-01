use crate::{
    error::expect_function_argument_amount, Macro, MacroInfo, Result, Table, Value, VariableMap,
};

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

        let mut table = argument[0].as_table()?.clone();

        for row in &argument[1..] {
            let row = row.as_list()?.clone();

            table.insert(row)?;
        }

        Ok(Value::Table(table))
    }
}

pub struct FindRow;

impl Macro for FindRow {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::find_row",
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
        let mut new_table = Table::new(table.column_names().clone());

        if let Some(row) = find {
            new_table.insert(row.clone()).unwrap();
        }

        Ok(Value::Table(new_table))
    }
}

pub struct Select;

impl Macro for Select {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "table::select",
            description: "Return a table with the selected columns.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_amount(argument.len(), 2)?;

        let table = argument[0].as_table()?;
        let column_names = argument[1]
            .as_list()?
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        let selected = table.select(&column_names);

        Ok(Value::Table(selected))
    }
}

pub struct Where;

impl Macro for Where {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "where",
            description: "Keep rows matching a predicate.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_amount(argument.len(), 2)?;

        let table = argument[0].as_table()?;
        let function = argument[1].as_function()?;
        let mut context = VariableMap::new();
        let mut new_table = Table::new(table.column_names().clone());

        for row in table.rows() {
            for (column_index, cell) in row.into_iter().enumerate() {
                let column_name = table.column_names().get(column_index).unwrap();

                context.set_value(column_name, cell.clone())?;
            }
            let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

            if keep_row {
                new_table.insert(row.clone())?;
            }
        }

        Ok(Value::Table(new_table))
    }
}
