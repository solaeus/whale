use std::fmt::{self, Display, Formatter};

use crate::{Error, Result, Value, VariableMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        use comfy_table::presets::UTF8_FULL;
        use comfy_table::Table as ComfyTable;

        let mut table = ComfyTable::new();
        table.load_preset(UTF8_FULL).set_header(&self.column_names);

        for row in &self.rows {
            table.add_row(row);
        }

        write!(f, "{table}")
    }
}
