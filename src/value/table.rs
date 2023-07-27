use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

use crate::{Error, Result, Value};
use serde::{Deserialize, Serialize};

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
            Value::Int(integer) => {
                let mut table = Table::new(vec!["integer".to_string()]);

                table.insert(vec![Value::Int(integer)]).unwrap();

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
                        if i == 0 {
                            let column_names = list
                                .iter()
                                .map(|value| value.as_string().unwrap())
                                .collect::<Vec<String>>();

                            table = Table::new(column_names);
                        } else {
                            table.insert(list).unwrap();
                        }
                    } else {
                        table.insert(vec![Value::Int(i as i64), value]).unwrap();
                    }
                }

                table
            }
            Value::Empty => todo!(),
            Value::Map(_) => todo!(),
            Value::Table(_) => todo!(),
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
