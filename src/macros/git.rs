use crate::{FunctionInfo, Macro, Result, Table, Value};

use git2::Repository;

pub struct Status;

impl Macro for Status {
    fn info(&self) -> FunctionInfo<'static> {
        FunctionInfo {
            identifier: "git::status",
            description: "Get the repository status for the current directory.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        argument.as_empty()?;

        let repo = Repository::open(".")?;
        let mut table = Table::new(vec![
            "path".to_string(),
            "status".to_string(),
            "staged".to_string(),
        ]);

        for entry in repo.statuses(None)?.into_iter() {
            let (status, staged) = {
                if entry.status().is_wt_new() {
                    ("created".to_string(), false)
                } else if entry.status().is_wt_deleted() {
                    ("deleted".to_string(), false)
                } else if entry.status().is_wt_modified() {
                    ("modified".to_string(), false)
                } else if entry.status().is_index_new() {
                    ("created".to_string(), true)
                } else if entry.status().is_index_deleted() {
                    ("deleted".to_string(), true)
                } else if entry.status().is_index_modified() {
                    ("modified".to_string(), true)
                } else if entry.status().is_ignored() {
                    continue;
                } else {
                    ("".to_string(), false)
                }
            };
            let path = entry.path().unwrap().to_string();

            table.insert(vec![
                Value::String(path),
                Value::String(status),
                Value::Boolean(staged),
            ])?;
        }

        Ok(Value::Table(table))
    }
}
