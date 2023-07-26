use crate::{Error, Result, Value};
use serde::{Deserialize, Serialize};

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
            let actual = row.get(column_index).unwrap();

            if actual == expected {
                return Some(row);
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
