use std::convert::TryFrom;

use crate::{Macro, MacroInfo, Result, Value};

use csv;
use json::JsonValue;

pub struct Get;

impl Macro for Get {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "get",
            description: "Extract a value from a collection.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_list()?;
        let collection = &argument[0];
        let index = &argument[1];

        if let Ok(list) = collection.as_list() {
            let index = index.as_int()?;
            let value = list.get(index as usize).unwrap_or(&Value::Empty);

            return Ok(value.clone());
        }

        if let Ok(table) = collection.as_table() {
            let index = index.as_int()?;
            let get_row = table.get(index as usize);

            if let Some(row) = get_row {
                return Ok(Value::List(row.clone()));
            }
        }

        Ok(Value::Empty)
    }
}

pub struct ToCsv;

impl Macro for ToCsv {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "to_csv",
            description: "Convert a value to a string of comma-separated values.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let mut buffer = Vec::new();
        let mut writer = csv::Writer::from_writer(&mut buffer);

        match argument {
            Value::String(string) => {
                writer.write_record(&[string])?;
            }
            Value::Float(float) => {
                writer.write_record(&[float.to_string()])?;
            }
            Value::Integer(integer) => {
                writer.write_record(&[integer.to_string()])?;
            }
            Value::Boolean(boolean) => {
                writer.write_record(&[boolean.to_string()])?;
            }
            Value::List(list) => {
                let string_list = list.iter().map(|value| value.to_string());

                writer.write_record(string_list)?;
            }
            Value::Empty => {}
            Value::Map(map) => {
                writer.write_record(map.inner().keys())?;
                writer.write_record(map.inner().values().map(|value| value.to_string()))?;
            }
            Value::Table(table) => {
                writer.write_record(table.column_names())?;

                for row in table.rows() {
                    let row_string = row.iter().map(|value| value.to_string());

                    writer.write_record(row_string)?;
                }
            }
            Value::Function(_) => todo!(),
        }

        writer.flush()?;

        Ok(Value::String(
            String::from_utf8_lossy(writer.get_ref()).to_string(),
        ))
    }
}

pub struct FromJson;

impl Macro for FromJson {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "from_json",
            description: "Convert JSON to a whale value.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let argument = argument.as_string()?;
        let json: JsonValue = json::parse(argument)?;
        let value = Value::try_from(json)?;

        Ok(value)
    }
}
