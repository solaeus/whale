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
    value::{value_type::ValueType, EmptyType, FloatType, IntType, TupleType, Value, EMPTY_VALUE},
    variable_map::VariableMap,
};

mod error;
mod functions;
mod interface;
mod operator;
mod table;
mod token;
mod tree;
mod value;
mod variable_map;
