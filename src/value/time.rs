use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    instant: DateTime<Utc>,
    timezone: Option<i32>,
}

impl Time {
    pub fn new(instant: DateTime<Utc>, timezone: Option<i32>) -> Self {
        Self { instant, timezone }
    }

    pub fn local(&self) -> String {
        if let Some(offset) = self.timezone {
            self.instant
                .with_timezone(&FixedOffset::west_opt(offset).unwrap())
                .to_rfc2822()
        } else {
            self.instant.to_string()
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.local())
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
