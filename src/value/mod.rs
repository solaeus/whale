use crate::{
    error::{Error, Result},
    Function, Table, VariableMap,
};

use comfy_table::{ContentArrangement, Table as ComfyTable};
use json::JsonValue;
use serde::{ser::SerializeTuple, Deserialize, Serialize, Serializer};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

pub mod function;
pub mod table;
pub mod value_type;
pub mod variable_map;

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Value {
    String(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    List(Vec<Value>),
    Empty,
    Map(VariableMap),
    Table(Table),
    Function(Function),
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Returns true if `self` is a `Value::Integer`.
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Integer` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::List`.
    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> Result<&String> {
        match self {
            Value::String(string) => Ok(string),
            value => Err(Error::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `i64`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> Result<i64> {
        match self {
            Value::Integer(i) => Ok(*i),
            value => Err(Error::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `f64`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float(&self) -> Result<f64> {
        match self {
            Value::Float(f) => Ok(*f),
            value => Err(Error::expected_float(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `f64`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `i64` to `f64`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> Result<f64> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Integer(i) => Ok(*i as f64),
            value => Err(Error::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> Result<bool> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(Error::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `Vec<Value>`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_list(&self) -> Result<&Vec<Value>> {
        match self {
            Value::List(list) => Ok(list),
            value => Err(Error::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `Vec<Value>` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_list(&self, len: usize) -> Result<Vec<Value>> {
        match self {
            Value::List(tuple) => {
                if tuple.len() == len {
                    Ok(tuple.clone())
                } else {
                    Err(Error::expected_fixed_len_tuple(len, self.clone()))
                }
            }
            value => Err(Error::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `Vec<Value>`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_map(&self) -> Result<VariableMap> {
        match self {
            Value::Map(map) => Ok(map.clone()),
            value => Err(Error::expected_map(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `Vec<Value>`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_table(&self) -> Result<Table> {
        match self {
            Value::Table(table) => Ok(table.clone()),
            value => Err(Error::expected_table(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_function(&self) -> Result<Function> {
        match self {
            Value::Function(function) => Ok(function.clone()),
            value => Err(Error::expected_function(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_empty(&self) -> Result<()> {
        match self {
            Value::Empty => Ok(()),
            value => Err(Error::expected_empty(value.clone())),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::String(left), Value::String(right)) => left.cmp(right),
            (Value::String(_), _) => Ordering::Greater,
            (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            (Value::Integer(_), _) => Ordering::Greater,
            (Value::Boolean(left), Value::Boolean(right)) => left.cmp(right),
            (Value::Boolean(_), _) => Ordering::Greater,
            (Value::Float(left), Value::Float(right)) => left.total_cmp(right),
            (Value::Float(_), _) => Ordering::Greater,
            (Value::List(left), Value::List(right)) => left.cmp(right),
            (Value::List(_), _) => Ordering::Greater,
            (Value::Map(left), Value::Map(right)) => left.cmp(right),
            (Value::Map(_), _) => Ordering::Greater,
            (Value::Table(left), Value::Table(right)) => left.cmp(right),
            (Value::Table(_), _) => Ordering::Greater,
            (Value::Function(left), Value::Function(right)) => left.cmp(right),
            (Value::Function(_), _) => Ordering::Greater,
            (Value::Empty, Value::Empty) => Ordering::Equal,
            (Value::Empty, _) => Ordering::Less,
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(inner) => serializer.serialize_str(&inner),
            Value::Float(inner) => serializer.serialize_f64(*inner),
            Value::Integer(inner) => serializer.serialize_i64(*inner),
            Value::Boolean(inner) => serializer.serialize_bool(*inner),
            Value::List(inner) => {
                let mut tuple = serializer.serialize_tuple(inner.len())?;

                for value in inner {
                    tuple.serialize_element(value)?;
                }

                tuple.end()
            }
            Value::Empty => todo!(),
            Value::Map(inner) => inner.serialize(serializer),
            Value::Table(inner) => inner.serialize(serializer),
            Value::Function(inner) => inner.serialize(serializer),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Float(float) => write!(f, "{}", float),
            Value::Integer(int) => write!(f, "{}", int),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::List(list) => {
                let mut comfy_table = ComfyTable::new();

                comfy_table
                    .load_preset("││──├─┼┤│    ┬┴╭╮╰╯")
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_header(0..list.len())
                    .add_row(list);

                write!(f, "{comfy_table}")
            }
            Value::Empty => write!(f, "()"),
            Value::Map(map) => write!(f, "{}", map),
            Value::Table(table) => table.fmt(f),
            Value::Function(string) => write!(f, "'{string}'"),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Value::Float(float)
    }
}

impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Value::Integer(int)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl From<Vec<Value>> for Value {
    fn from(tuple: Vec<Value>) -> Self {
        Value::List(tuple)
    }
}

impl From<Value> for Result<Value> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

impl TryFrom<JsonValue> for Value {
    type Error = Error;

    fn try_from(json_value: JsonValue) -> Result<Self> {
        use JsonValue::*;

        match json_value {
            Null => Ok(Value::Empty),
            Short(short) => Ok(Value::String(short.to_string())),
            String(string) => Ok(Value::String(string)),
            Number(number) => Ok(Value::Float(f64::from(number))),
            Boolean(boolean) => Ok(Value::Boolean(boolean)),
            Object(object) => {
                let mut map = VariableMap::new();

                for (key, node_value) in object.iter() {
                    let value = Value::try_from(node_value)?;

                    map.set_value(key, value)?;
                }

                Ok(Value::Map(map))
            }
            Array(array) => {
                let mut list = Vec::new();

                for json_value in array {
                    let value = Value::try_from(json_value)?;

                    list.push(value);
                }

                Ok(Value::List(list))
            }
        }
    }
}

impl TryFrom<&JsonValue> for Value {
    type Error = Error;

    fn try_from(json_value: &JsonValue) -> Result<Self> {
        use JsonValue::*;

        match json_value {
            Null => Ok(Value::Empty),
            Short(short) => Ok(Value::String(short.to_string())),
            String(string) => Ok(Value::String(string.clone())),
            Number(number) => Ok(Value::Float(f64::from(*number))),
            Boolean(boolean) => Ok(Value::Boolean(*boolean)),
            Object(object) => {
                let mut map = VariableMap::new();

                for (key, node_value) in object.iter() {
                    let value = Value::try_from(node_value)?;

                    map.set_value(key, value)?;
                }

                Ok(Value::Map(map))
            }
            Array(array) => {
                let mut list = Vec::new();

                for json_value in array {
                    let value = Value::try_from(json_value)?;

                    list.push(value);
                }

                Ok(Value::List(list))
            }
        }
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::String(value) = value {
            Ok(value)
        } else {
            Err(Error::ExpectedString { actual: value })
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::Float(value) = value {
            Ok(value)
        } else {
            Err(Error::ExpectedFloat { actual: value })
        }
    }
}

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::Integer(value) = value {
            Ok(value)
        } else {
            Err(Error::ExpectedInt { actual: value })
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::Boolean(value) = value {
            Ok(value)
        } else {
            Err(Error::ExpectedBoolean { actual: value })
        }
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::List(value) = value {
            Ok(value)
        } else {
            Err(Error::ExpectedTuple { actual: value })
        }
    }
}

impl TryFrom<Value> for () {
    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Value::Empty = value {
            Ok(())
        } else {
            Err(Error::ExpectedEmpty { actual: value })
        }
    }
}
