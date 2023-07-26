/*!
The whale language.
*/
#![forbid(unsafe_code)]

pub use crate::{
    error::{Error, Result},
    functions::*,
    interface::*,
    operator::Operator,
    token::PartialToken,
    tree::Node,
    value::{
        table::Table, value_type::ValueType, variable_map::VariableMap, EmptyType, FloatType,
        IntType, TupleType, Value, EMPTY_VALUE,
    },
};

mod error;
mod functions;
mod interface;
mod operator;
mod token;
mod tree;
mod value;
