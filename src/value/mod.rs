use crate::{
    error::{Error, Result},
    Table, VariableMap,
};

use serde::{
    ser::{SerializeMap, SerializeTuple},
    Deserialize, Serialize, Serializer,
};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

pub mod table;
pub mod value_type;
pub mod variable_map;

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum Value {
    /// A string value.
    String(String),
    /// A float value.
    Float(f64),
    /// An integer value.
    Int(i64),
    /// A boolean value.
    Boolean(bool),
    /// A tuple value.
    List(Vec<Value>),
    /// An empty value.
    Empty,
    /// Collection of key-value pairs.
    Map(VariableMap),
    /// Structured collection of related items.
    Table(Table),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(inner) => serializer.serialize_str(&inner),
            Value::Float(inner) => serializer.serialize_f64(*inner),
            Value::Int(inner) => serializer.serialize_i64(*inner),
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
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        matches!(self, Value::List(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> Result<String> {
        match self {
            Value::String(string) => Ok(string.clone()),
            value => Err(Error::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `i64`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> Result<i64> {
        match self {
            Value::Int(i) => Ok(*i),
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
            Value::Int(i) => Ok(*i as f64),
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
    pub fn as_tuple(&self) -> Result<Vec<Value>> {
        match self {
            Value::List(tuple) => Ok(tuple.clone()),
            value => Err(Error::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `Vec<Value>` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_tuple(&self, len: usize) -> Result<Vec<Value>> {
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
    pub fn as_empty(&self) -> Result<()> {
        match self {
            Value::Empty => Ok(()),
            value => Err(Error::expected_empty(value.clone())),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::String(string) => write!(f, "\"{}\"", string),
            Value::Float(float) => write!(f, "{}", float),
            Value::Int(int) => write!(f, "{}", int),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::List(tuple) => {
                write!(f, "(")?;
                let mut once = false;
                for value in tuple {
                    if once {
                        write!(f, ", ")?;
                    } else {
                        once = true;
                    }
                    value.fmt(f)?;
                }
                write!(f, ")")
            }
            Value::Empty => write!(f, "()"),
            Value::Map(map) => write!(f, "{}", map),
            Value::Table(table) => table.fmt(f),
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
        Value::Int(int)
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
        if let Value::Int(value) = value {
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

#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn test_value_conversions() {
        assert_eq!(
            Value::from("string").as_string(),
            Ok(String::from("string"))
        );
        assert_eq!(Value::from(3).as_int(), Ok(3));
        assert_eq!(Value::from(3.3).as_float(), Ok(3.3));
        assert_eq!(Value::from(true).as_boolean(), Ok(true));
        assert_eq!(Value::from(Vec::new()).as_tuple(), Ok(Vec::new()));
    }

    #[test]
    fn test_value_checks() {
        assert!(Value::from("string").is_string());
        assert!(Value::from(3).is_int());
        assert!(Value::from(3.3).is_float());
        assert!(Value::from(true).is_boolean());
        assert!(Value::from(Vec::new()).is_tuple());
    }
}
