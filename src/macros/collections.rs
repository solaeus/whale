//! Macros for collection values: strings, lists, maps and tables.

use crate::{Error, Macro, MacroInfo, Result, Table, Value, ValueType, VariableMap};

pub struct CreateTable;

impl Macro for CreateTable {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "create_table",
            description: "Define a new table with a list of column names and list of rows.",
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let column_name_inputs = argument[0].as_list()?;
        let mut column_names = Vec::with_capacity(column_name_inputs.len());

        for name in column_name_inputs {
            column_names.push(name.as_string()?.clone());
        }

        let column_count = column_names.len();
        let rows = argument[1].as_list()?;
        let mut table = Table::new(column_names);

        for row in rows {
            let row = row.as_fixed_len_list(column_count)?;

            table.insert(row.clone()).unwrap();
        }

        Ok(Value::Table(table))
    }
}

pub struct Rows;

impl Macro for Rows {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "rows",
            description: "Extract a table's rows as a list.",
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let table = argument.as_table()?;

        let rows = table
            .rows()
            .iter()
            .map(|row| Value::List(row.clone()))
            .collect();

        Ok(Value::List(rows))
    }
}

pub struct Get;

impl Macro for Get {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "get",
            description: "Retrieve a value from a collection.",
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        let collection = &argument[0];
        let index = argument[1].as_int()?;

        if let Ok(list) = collection.as_list() {
            if let Some(value) = list.get(index as usize) {
                return Ok(value.clone());
            } else {
                return Ok(Value::Empty);
            }
        }

        Err(Error::TypeError {
            expected: &[ValueType::List, ValueType::Map, ValueType::Table],
            actual: collection.clone(),
        })
    }
}

pub struct Insert;

impl Macro for Insert {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "insert",
            description: "Add new rows to a table.",
            group: "collections",
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
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let arguments = argument.as_fixed_len_list(2)?;
        let collection = &arguments[0];

        if let Value::List(list) = collection {
            let mut selected = Vec::new();

            let index = arguments[1].as_int()?;
            let value = list.get(index as usize);

            if let Some(value) = value {
                selected.push(value.clone());
                return Ok(Value::List(selected));
            } else {
                return Ok(Value::List(selected));
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
            return Err(Error::TypeError {
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

        if let Value::Table(table) = collection {
            let selected = table.select(&column_names);

            return Ok(Value::Table(selected));
        }

        Err(Error::TypeError {
            expected: &[ValueType::List, ValueType::Map, ValueType::Table],
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
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;

        Error::expected_minimum_function_argument_amount(
            self.info().identifier,
            2,
            argument.len(),
        )?;

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
            group: "collections",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument_list = argument.as_list()?;
        Error::expect_function_argument_amount(self.info().identifier, argument_list.len(), 2)?;

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

        if let Ok(map) = collection.as_map() {
            let mut context = VariableMap::new();
            let mut new_map = VariableMap::new();

            for (key, value) in map.inner() {
                if let Ok(map) = value.as_map() {
                    for (key, value) in map.inner() {
                        context.set_value(key, value.clone())?;
                    }
                } else {
                    context.set_value("input", value.clone())?;
                }

                let keep_row = function.run_with_context(&mut context)?.as_boolean()?;

                if keep_row {
                    new_map.set_value(key, value.clone())?;
                }
            }

            return Ok(Value::Map(new_map));
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

        Err(Error::TypeError {
            expected: &[ValueType::List, ValueType::Map, ValueType::Table],
            actual: collection.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Function;

    use super::*;

    #[test]
    fn where_from_non_collections() {
        Where
            .run(&Value::List(vec![
                Value::Integer(1),
                Value::Function(Function::new("input == 1")),
            ]))
            .unwrap_err();
        Where
            .run(&Value::List(vec![
                Value::Float(1.0),
                Value::Function(Function::new("input == 1.0")),
            ]))
            .unwrap_err();
        Where
            .run(&Value::List(vec![
                Value::Boolean(true),
                Value::Function(Function::new("input == true")),
            ]))
            .unwrap_err();
    }

    #[test]
    fn where_from_list() {
        let arguments = Value::List(vec![
            Value::List(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Function(Function::new("input == 1")),
        ]);
        let select = Where.run(&arguments).unwrap();

        assert_eq!(Value::List(vec![Value::Integer(1)]), select);
    }

    #[test]
    fn where_from_map() {
        let mut map = VariableMap::new();

        map.set_value("foo", Value::Integer(1)).unwrap();
        map.set_value("bar", Value::Integer(2)).unwrap();

        let arguments = Value::List(vec![
            Value::Map(map),
            Value::Function(Function::new("input == 1")),
        ]);
        let select = Where.run(&arguments).unwrap();

        let mut map = VariableMap::new();

        map.set_value("foo", Value::Integer(1)).unwrap();

        assert_eq!(Value::Map(map), select);
    }

    #[test]
    fn where_from_table() {
        let mut table = Table::new(vec!["foo".to_string(), "bar".to_string()]);

        table
            .insert(vec![Value::Integer(1), Value::Integer(2)])
            .unwrap();
        table
            .insert(vec![Value::Integer(3), Value::Integer(4)])
            .unwrap();

        let arguments = Value::List(vec![
            Value::Table(table),
            Value::Function(Function::new("foo == 1")),
        ]);
        let select = Where.run(&arguments).unwrap();
        let mut table = Table::new(vec!["foo".to_string(), "bar".to_string()]);

        table
            .insert(vec![Value::Integer(1), Value::Integer(2)])
            .unwrap();

        assert_eq!(Value::Table(table), select);
    }

    #[test]
    fn select_from_non_collections() {
        Select
            .run(&Value::List(vec![Value::Integer(1), Value::Integer(1)]))
            .unwrap_err();
        Select
            .run(&Value::List(vec![Value::Float(1.0), Value::Float(1.0)]))
            .unwrap_err();
        Select
            .run(&Value::List(vec![
                Value::Boolean(true),
                Value::Boolean(true),
            ]))
            .unwrap_err();
    }

    #[test]
    fn select_from_list() {
        let arguments = Value::List(vec![
            Value::List(vec![Value::Integer(1), Value::Integer(2)]),
            Value::Integer(0),
        ]);
        let select = Select.run(&arguments).unwrap();

        assert_eq!(Value::List(vec![Value::Integer(1)]), select);
    }

    #[test]
    fn select_from_map() {
        let mut map = VariableMap::new();

        map.set_value("foo", Value::Integer(1)).unwrap();
        map.set_value("bar", Value::Integer(2)).unwrap();

        let arguments = Value::List(vec![Value::Map(map), Value::String("foo".to_string())]);
        let select = Select.run(&arguments).unwrap();

        let mut map = VariableMap::new();

        map.set_value("foo", Value::Integer(1)).unwrap();

        assert_eq!(Value::Map(map), select);
    }

    #[test]
    fn select_from_table() {
        let mut table = Table::new(vec!["foo".to_string(), "bar".to_string()]);

        table
            .insert(vec![Value::Integer(1), Value::Integer(2)])
            .unwrap();

        let arguments = Value::List(vec![Value::Table(table), Value::String("foo".to_string())]);
        let select = Select.run(&arguments).unwrap();

        let mut table = Table::new(vec!["foo".to_string()]);

        table.insert(vec![Value::Integer(1)]).unwrap();

        assert_eq!(Value::Table(table), select);
    }
}
