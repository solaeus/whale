use crate::{Macro, MacroInfo, Result, Value};

pub struct Now;

impl Macro for Now {
    fn info(&self) -> MacroInfo<'static> {
        MacroInfo {
            identifier: "now",
            description: "Return the current time.",
            group: "time",
        }
    }

    fn run(&self, argument: &crate::Value) -> Result<Value> {
        argument.as_empty()?;
    }
}
