///! Serde (de-)serializer for ISO 8601 "date and time"s without timezone
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Converts a `NaiveDateTime` to its `String` representation
///
/// # Arguments
///
/// - `datetime`: `NaiveDateTime` which shall be converted
///
/// # Returns
///
/// The `String` representation
fn time_to_json(datetime: NaiveDateTime) -> String {
    DateTime::<Utc>::from_utc(datetime, Utc).to_rfc3339()
}

/// Serde-serializer for a `NaiveDateTime` field
///
/// # Arguments
///
/// - `time`: `NaiveDateTime` property that has to be serialized
/// - `serializer`: The serializer
///
/// # Returns
///
/// The serialized representation
///
/// # Bails out when
///
/// - the value can't be serialized
pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
    time_to_json(time.clone()).serialize(serializer)
}

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
    Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(D::Error::custom)?)
}
