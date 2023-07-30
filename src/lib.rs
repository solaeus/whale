#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

pub use crate::{
    error::{Error, Result},
    interface::*,
    macros::*,
    operator::Operator,
    token::PartialToken,
    tree::Node,
    value::{
        function::Function, table::Table, value_type::ValueType, variable_map::VariableMap, Value,
    },
};

mod error;
mod interface;
mod macros;
mod operator;
mod token;
mod tree;
mod value;
