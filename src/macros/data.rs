use crate::{FunctionInfo, Macro, Result, Value};

use csv;

pub struct Csv;

impl Macro for Csv {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "data::to_csv",
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
