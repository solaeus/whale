use nu_table::{Cell, NuTable, TableConfig};

use crate::{Error, Result, Value, VariableMap};
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

    pub fn reserve(&mut self, additional: usize) {
        self.rows.reserve(additional);
    }

    pub fn column_names(&self) -> &Vec<String> {
        &self.column_names
    }

    pub fn rows(&self) -> &Vec<Vec<Value>> {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
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

    pub fn remove(&mut self, index: usize) -> Result<()> {
        self.rows.remove(index);

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&Vec<Value>> {
        self.rows.get(index)
    }

    pub fn select(&self, column_names: &[String]) -> Table {
        let matching_column_indexes = column_names
            .iter()
            .filter_map(|name| self.get_column_index(name))
            .collect::<Vec<usize>>();
        let mut new_table = Table::new(column_names.to_vec());

        for row in &self.rows {
            let mut new_row = Vec::new();

            for (i, value) in row.iter().enumerate() {
                if matching_column_indexes.contains(&i) {
                    new_row.push(value.clone());
                }
            }

            new_table.insert(new_row).unwrap();
        }

        new_table
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

    pub fn filter(&self, column_name: &str, expected: &Value) -> Option<Table> {
        let mut filtered = Table::new(self.column_names.clone());
        let column_index = self.get_column_index(column_name)?;

        for row in &self.rows {
            let actual = row.get(column_index).unwrap();

            if actual == expected {
                let _ = filtered.insert(row.clone());
            }
        }

        Some(filtered)
    }

    pub fn get_column_index(&self, column_name: &str) -> Option<usize> {
        let column_names = &self.column_names;
        for (i, column) in column_names.iter().enumerate() {
            if column == column_name {
                return Some(i);
            }
        }
        None
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let rows: Vec<Vec<Cell>> = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|value| Cell::new(value.to_string()))
                    .collect()
            })
            .collect();
        let table = NuTable::from(rows);

        let display = table.draw(TableConfig::default(), 80).unwrap();

        write!(f, "{display}")
    }
}

impl From<&Value> for Table {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(string) => {
                let mut table = Table::new(vec!["string".to_string()]);

                table
                    .insert(vec![Value::String(string.to_string())])
                    .unwrap();

                table
            }
            Value::Float(float) => {
                let mut table = Table::new(vec!["float".to_string()]);

                table.insert(vec![Value::Float(*float)]).unwrap();

                table
            }
            Value::Integer(integer) => {
                let mut table = Table::new(vec!["integer".to_string()]);

                table.insert(vec![Value::Integer(*integer)]).unwrap();

                table
            }
            Value::Boolean(boolean) => {
                let mut table = Table::new(vec!["boolean".to_string()]);

                table.insert(vec![Value::Boolean(*boolean)]).unwrap();

                table
            }
            Value::List(list) => Self::from(list),
            Value::Empty => Table::new(Vec::with_capacity(0)),
            Value::Map(map) => Self::from(map),
            Value::Table(table) => table.clone(),
            Value::Function(function) => {
                let mut table = Table::new(vec!["function".to_string()]);

                table
                    .insert(vec![Value::Function(function.clone())])
                    .unwrap();

                table
            }
        }
    }
}

impl From<&Vec<Value>> for Table {
    fn from(list: &Vec<Value>) -> Self {
        let mut table = Table::new(vec!["index".to_string(), "item".to_string()]);

        for (i, value) in list.iter().enumerate() {
            if let Ok(list) = value.as_list() {
                table.insert(list.clone()).unwrap();
            } else {
                table
                    .insert(vec![Value::Integer(i as i64), value.clone()])
                    .unwrap();
            }
        }

        table
    }
}

impl From<&VariableMap> for Table {
    fn from(map: &VariableMap) -> Self {
        let keys = map.inner().keys().cloned().collect();
        let values = map.inner().values().cloned().collect();
        let mut table = Table::new(keys);

        table
            .insert(values)
            .expect("Failed to create Table from Map. This is a no-op.");

        table
    }
}

impl Eq for Table {}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.column_names != other.column_names {
            return false;
        }

        for self_row in &self.rows {
            for other_row in &other.rows {
                if self_row != other_row {
                    return false;
                }
            }
        }

        true
    }
}

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
