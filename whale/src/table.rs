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
}
