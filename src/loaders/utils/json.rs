use anyhow::{Context, Result};
use serde_json::Value;

/// Loads a JSON file and parses it to a JSON-`Value`
///
/// # Parameters
///
/// 1. File path to the input
///
/// # Returns
///
/// The parsed JSON-`Value`
///
/// # Bails out when
///
/// - Input file can't be read
/// - Input file can't be parsed
pub fn load_and_parse(filename: String) -> Result<Value> {
    println!("Reading input file");
    let text =
        std::fs::read_to_string(&filename).with_context(|| format!("Can't read {}", &filename))?;
    Ok(serde_json::from_str(&text).with_context(|| format!("Unable to parse '{}'", &filename))?)
}
