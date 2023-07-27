use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    marker::PhantomData,
};

use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize,
};

use crate::{call_builtin_function, value::Value, Error, Result};

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
        if identifier == "context" {
            return Ok(Value::Map(self.clone()));
        }

        call_builtin_function(identifier, argument)
    }

    pub fn get_value(&self, identifier: &str) -> Result<Option<Value>> {
        let split = identifier.split_once(".");

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
                let function_result = self.call_function(identifier, &Value::Empty)?;
                Ok(Some(function_result))
            }
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

    pub fn set_value(&mut self, identifier: &str, value: Value) -> Result<()> {
        let split = identifier.split_once(".");
        let value = if let Ok(result) = self.call_function(identifier, &value) {
            result
        } else {
            value
        };

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
}

impl Display for VariableMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use comfy_table::presets::UTF8_FULL;
        use comfy_table::Table as ComfyTable;

        let mut table = ComfyTable::new();
        table.load_preset(UTF8_FULL);

        table.set_header(self.variables.keys());
        table.add_row(self.variables.values());

        write!(f, "{table}")
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
    // The type that our Visitor is going to produce.
    type Value = VariableMap;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("VariableMap of key-value pairs.")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> std::result::Result<VariableMap, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = VariableMap::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.set_value(key, value);
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
