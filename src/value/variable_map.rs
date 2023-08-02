use comfy_table::{ContentArrangement, Table as ComfyTable};
use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize,
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    marker::PhantomData,
};

use crate::{value::Value, Error, Result, MACRO_LIST};

/// A context that stores its mappings in hash maps.
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct VariableMap {
    variables: BTreeMap<String, Value>,
}

impl VariableMap {
    /// Creates a new instace.
    pub fn new() -> Self {
        VariableMap {
            variables: BTreeMap::new(),
        }
    }

    pub fn call_function(&self, identifier: &str, argument: &Value) -> Result<Value> {
        for macro_item in MACRO_LIST {
            if identifier == macro_item.info().identifier {
                return macro_item.run(argument);
            }
        }

        for (key, value) in &self.variables {
            if identifier == key {
                if let Ok(function) = value.as_function() {
                    return function.run();
                }
            }
        }

        let mut split = identifier.split(':').rev();

        if let (Some(function_identifier), Some(variable_identifier)) = (split.next(), split.next())
        {
            if function_identifier.contains(':') {
                return self.call_function(function_identifier, argument);
            }

            if variable_identifier.split_once('.').is_some() {
                let value = self.get_value(variable_identifier)?.unwrap_or(Value::Empty);

                return self.call_function(function_identifier, &value);
            }

            if let Some(value) = self.get_value(variable_identifier)? {
                if argument.is_empty() {
                    return self.call_function(function_identifier, &value);
                }

                let list = Value::List(vec![value, argument.clone()]);

                return self.call_function(function_identifier, &list);
            }
        }

        Err(Error::FunctionIdentifierNotFound(identifier.to_string()))
    }

    pub fn get_value(&self, identifier: &str) -> Result<Option<Value>> {
        let split = identifier.split_once('.');

        if let Some((identifier, next_identifier)) = split {
            if let Some(value) = self.variables.get(identifier) {
                if let Value::Map(map) = value {
                    map.get_value(next_identifier)
                } else {
                    Err(Error::ExpectedMap {
                        actual: value.clone(),
                    })
                }
            } else {
                Ok(None)
            }
        } else {
            let value = self.variables.get(identifier);

            if let Some(value) = value {
                Ok(Some(value.clone()))
            } else {
                Ok(None)
            }
        }
    }

    pub fn set_value(&mut self, identifier: &str, value: Value) -> Result<()> {
        let split = identifier.split_once('.');

        if let Some((map_name, next_identifier)) = split {
            let get_value = self.variables.get_mut(map_name);

            if let Some(map_value) = get_value {
                if let Value::Map(map) = map_value {
                    map.set_value(next_identifier, value)
                } else {
                    Err(Error::ExpectedMap {
                        actual: map_value.clone(),
                    })
                }
            } else {
                let mut new_map = VariableMap::new();

                new_map.set_value(next_identifier, value)?;

                self.variables
                    .insert(map_name.to_string(), Value::Map(new_map));

                Ok(())
            }
        } else {
            self.variables.insert(identifier.to_string(), value);

            Ok(())
        }
    }

    /// Returns a reference to the inner BTreeMap.
    pub fn inner(&self) -> &BTreeMap<String, Value> {
        &self.variables
    }

    /// Returns the number of stored variables.
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Returns true if the length is zero.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl Default for VariableMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for VariableMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut comfy_table = ComfyTable::new();

        comfy_table
            .load_preset("││──├─┼┤│    ┬┴╭╮╰╯")
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(self.variables.keys())
            .add_row(self.variables.values());

        write!(f, "{comfy_table}")
    }
}

impl Serialize for VariableMap {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.variables.len()))?;

        for (key, value) in &self.variables {
            map.serialize_entry(&key, &value)?;
        }

        map.end()
    }
}

struct VariableMapVisitor {
    marker: PhantomData<fn() -> VariableMap>,
}

impl VariableMapVisitor {
    fn new() -> Self {
        VariableMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for VariableMapVisitor {
    type Value = VariableMap;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("VariableMap of key-value pairs.")
    }

    fn visit_map<M>(self, mut access: M) -> std::result::Result<VariableMap, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = VariableMap::new();

        while let Some((key, value)) = access.next_entry()? {
            map.set_value(key, value)
                .expect("Failed to deserialize VariableMap. This is a no-op.");
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for VariableMap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(VariableMapVisitor::new())
    }
}
