/// Serde deserializer for ISO 8601 "date and time"s without timezone
use chrono::NaiveDateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

/// Serde-deserializer for a `NaiveDateTime` field
///
/// # Arguments
///
/// - `deserializer`: The Deserializer
///
/// # Returns
///
/// The deserialized `NaiveDateTime`
///
/// # Bails out when
///
/// - the value can't be deserialized
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
    let time: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(Error::custom)
}
