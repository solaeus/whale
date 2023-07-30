#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use crate::{
    error::{Error, Result},
    functions::*,
    interface::*,
    operator::Operator,
    token::PartialToken,
    tree::Node,
    value::{
        function::Function, table::Table, value_type::ValueType, variable_map::VariableMap, Value,
    },
};

mod error;
mod functions;
mod interface;
mod operator;
mod token;
mod tree;
mod value;
