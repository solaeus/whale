use crate::{Error, Result, Value};

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Cell, Color, Table as ComfyTable};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    column_names: Vec<String>,
    rows: Vec<Vec<Value>>,
}

impl Table {
    pub fn new(column_names: Vec<String>) -> Self {
        Table {
            column_names,
            rows: Vec::new(),
        }
    }

    pub fn column_names(&self) -> &Vec<String> {
        &self.column_names
    }

    pub fn rows(&self) -> &Vec<Vec<Value>> {
        &self.rows
    }

    pub fn sort(&mut self) {
        self.rows.sort();
    }

    pub fn insert(&mut self, row: Vec<Value>) -> Result<()> {
        if row.len() != self.column_names.len() {
            return Err(Error::WrongColumnAmount {
                expected: self.column_names.len(),
                actual: row.len(),
            });
        }

        self.rows.push(row);

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&Vec<Value>> {
        self.rows.get(index)
    }

    pub fn get_where(&self, column_name: &str, expected: &Value) -> Option<&Vec<Value>> {
        let column_index = self.get_column_index(column_name)?;

        for row in &self.rows {
            if let Some(actual) = row.get(column_index) {
                if actual == expected {
                    return Some(row);
                }
            }
        }

        None
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        let column_names = &self.column_names;
        for (i, column) in column_names.into_iter().enumerate() {
            if column == column_name {
                return Some(i);
            }
        }
        None
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut table = ComfyTable::new();

        table
            .load_preset("││──├─┼┤│    ┬┴╭╮╰╯")
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(&self.column_names)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

        for row in &self.rows {
            let row = row.iter().map(|value| {
                let mut cell = Cell::new(value.to_string()).bg(Color::Rgb {
                    r: 40,
                    g: 40,
                    b: 40,
                });

                if value.is_string() {
                    cell = cell.fg(Color::Green);
                }
                if value.is_integer() {
                    cell = cell.fg(Color::Blue);
                }
                if value.is_boolean() {
                    cell = cell.fg(Color::Red);
                }
                if value.is_function() {
                    cell = cell.fg(Color::Cyan);
                }

                cell
            });

            table.add_row(row);
        }

        write!(f, "{table}")
    }
}

impl From<Value> for Table {
    fn from(value: Value) -> Self {
        match value {
            Value::String(string) => {
                let mut table = Table::new(vec!["string".to_string()]);

                table.insert(vec![Value::String(string)]).unwrap();

                table
            }
            Value::Float(float) => {
                let mut table = Table::new(vec!["float".to_string()]);

                table.insert(vec![Value::Float(float)]).unwrap();

                table
            }
            Value::Integer(integer) => {
                let mut table = Table::new(vec!["integer".to_string()]);

                table.insert(vec![Value::Integer(integer)]).unwrap();

                table
            }
            Value::Boolean(boolean) => {
                let mut table = Table::new(vec!["boolean".to_string()]);

                table.insert(vec![Value::Boolean(boolean)]).unwrap();

                table
            }
            Value::List(list) => {
                let mut table = Table::new(vec!["index".to_string(), "item".to_string()]);

                for (i, value) in list.into_iter().enumerate() {
                    if let Ok(list) = value.as_list() {
                        table.insert(list.clone()).unwrap();
                    } else {
                        table.insert(vec![Value::Integer(i as i64), value]).unwrap();
                    }
                }

                table
            }
            Value::Empty => Table::new(Vec::with_capacity(0)),
            Value::Map(map) => {
                let keys = map.inner().keys().cloned().collect();
                let values = map.inner().values().cloned().collect();
                let mut table = Table::new(keys);

                table
                    .insert(values)
                    .expect("Failed to create Table from Map. This is a no-op.");

                table
            }
            Value::Table(table) => table,
            Value::Function(function) => {
                let mut table = Table::new(vec!["function".to_string()]);

                table.insert(vec![Value::Function(function)]).unwrap();

                table
            }
        }
    }
}

impl Eq for Table {}

impl PartialOrd for Table {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.column_names.partial_cmp(&other.column_names)
    }
}

impl Ord for Table {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.column_names.cmp(&other.column_names)
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        self.column_names == other.column_names
    }
}
