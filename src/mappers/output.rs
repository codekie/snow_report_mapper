use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

/// Context for the errors that may occur during parsing
const ERR_MSG_UNABLE_TO_PROCESS_INPUT: &str = "Unable to process input file";

/// Represents an OpenAI model training entry
#[derive(Serialize, Deserialize, Debug)]
pub struct OutputEntry<'a> {
    /// Title of the ServiceNOW incident
    name: &'a str,
    /// Group to which the incident was assigned to
    assignment_group: &'a str,
}

/// Maps the ServiceNOW report data to OpenAI training data
///
/// # Parameters
///
/// 1. The parsed ServiceNOW report
///
/// # Returns
///
/// The mapped OpenAI training data
///
/// # Bails out when
///
/// - Data can't be mapped
pub fn map_data(input_data: &Value) -> Result<Vec<OutputEntry>> {
    let mut result: Vec<OutputEntry> = Vec::new();
    let entries: &Vec<Value> = input_data["records"]
        .as_array()
        .with_context(|| ERR_MSG_UNABLE_TO_PROCESS_INPUT)?;
    println!("{} entries found", entries.len());
    println!("Mapping data");
    for entry in entries {
        result.push(OutputEntry {
            name: entry["short_description"]
                .as_str()
                .with_context(|| ERR_MSG_UNABLE_TO_PROCESS_INPUT)?,
            assignment_group: entry["assignment_group"]
                .as_str()
                .with_context(|| ERR_MSG_UNABLE_TO_PROCESS_INPUT)?,
        })
    }
    Ok(result)
}
