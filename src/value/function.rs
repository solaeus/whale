use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{eval, Result, Value};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Function(String);

impl Function {
    pub fn new(body: String) -> Self {
        Function(body)
    }

    pub fn run(&self) -> Result<Value> {
        eval(&self.0)
    }
}

impl From<String> for Function {
    fn from(value: String) -> Self {
        Function(value)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.0)
    }
}
