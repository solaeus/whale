use std::process::{Command, Stdio};

use crate::{Macro, Result, Value};

pub struct Download;

impl Macro for Download {
    fn info(&self) -> crate::MacroInfo<'static> {
        crate::MacroInfo {
            identifier: "network::download",
            description: "Download a file from a URL.",
        }
    }

    fn run(&self, argument: &Value) -> Result<Value> {
        let url = argument.as_string()?;
        let script = format!("curl --tlsv1.2 -sSf {url}");
        let download = Command::new("fish")
            .arg("-c")
            .arg(script)
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()?
            .stdout;

        Ok(Value::String(
            String::from_utf8_lossy(&download).to_string(),
        ))
    }
}
