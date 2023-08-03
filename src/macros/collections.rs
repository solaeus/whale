//! Macros for collection values: strings, lists, maps and tables.

use crate::{
    error::expect_function_argument_length, Error, Macro, MacroInfo, Result, Table, Value,
    ValueType, VariableMap,
};

pub struct CreateTable;

impl Macro for CreateTable {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "create_table",
            description: "Define a new table with a list of column names and list of rows.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_fixed_len_list(2)?;

        let column_names = argument[0]
            .as_list()?
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        let column_count = column_names.len();
        let rows = argument[1].as_list()?;
        let mut table = Table::new(column_names);

        for row in rows {
            let row = row.as_list()?.clone();

            expect_function_argument_length(row.len(), column_count)?;

            table.insert(row).unwrap();
        }

        Ok(Value::Table(table))
    }
}

pub struct Insert;

impl Macro for Insert {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "insert_rows",
            description: "Add new rows to a table.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let new_rows = &argument[1..];
        let mut table = argument[0].as_table()?.clone();

        table.reserve(new_rows.len());

        for row in new_rows {
            let row = row.as_list()?.clone();

            table.insert(row)?;
        }

        Ok(Value::Table(table))
    }
}

pub struct Select;

impl Macro for Select {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "select",
            description: "Extract one or more values based on their key.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let arguments = argument.as_fixed_len_list(2)?;
        let collection = &arguments[0];

        if let Value::List(list) = collection {
            let index = arguments[1].as_int()?;
            let value = list.get(index as usize);

            if let Some(value) = value {
                return Ok(value.clone());
            } else {
                return Ok(Value::Empty);
            }
        }

        let mut column_names = Vec::new();

        if let Value::List(columns) = &arguments[1] {
            for column in columns {
                let name = column.as_string()?;

                column_names.push(name.clone());
            }
        } else if let Value::String(column) = &arguments[1] {
            column_names.push(column.clone());
        } else {
            return Err(Error::ExpectedValueType {
                expected: &[ValueType::String, ValueType::List],
                actual: arguments[1].clone(),
            });
        };

        if let Value::Map(map) = collection {
            let mut selected = VariableMap::new();

            for (key, value) in map.inner() {
                if column_names.contains(key) {
                    selected.set_value(key, value.clone())?;
                }
            }

            return Ok(Value::Map(selected));
        }

        Err(Error::ExpectedValueType {
            expected: &[ValueType::List, ValueType::Map],
            actual: collection.clone(),
        })
    }
}

pub struct ForEach;

impl Macro for ForEach {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "for_each",
            description: "Run an operation on every item in a collection.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        expect_function_argument_length(argument.len(), 2)?;

        let table = argument[0].as_table()?;
        let columns = argument[1].as_list()?;
        let mut column_names = Vec::new();

        for column in columns {
            let name = column.as_string()?;

            column_names.push(name.clone());
        }

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
        let argument_list = argument.as_list()?;
        expect_function_argument_length(argument_list.len(), 2)?;

        let collection = &argument_list[0];
        let function = argument_list[1].as_function()?;

        if let Ok(list) = collection.as_list() {
            let mut context = VariableMap::new();
            let mut new_list = Vec::new();

            for value in list {
                context.set_value("input", value.clone())?;
                let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

                if keep_row {
                    new_list.push(value.clone());
                }
            }

            return Ok(Value::List(new_list));
        }

        if let Ok(table) = collection.as_table() {
            let mut context = VariableMap::new();
            let mut new_table = Table::new(table.column_names().clone());

            for row in table.rows() {
                for (column_index, cell) in row.iter().enumerate() {
                    let column_name = table.column_names().get(column_index).unwrap();

                    context.set_value(column_name, cell.clone())?;
                }
                let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

                if keep_row {
                    new_table.insert(row.clone())?;
                }
            }

            return Ok(Value::Table(new_table));
        }

        Err(Error::ExpectedValueType {
            expected: &[ValueType::List, ValueType::Table],
            actual: collection.clone(),
        })
    }
}
