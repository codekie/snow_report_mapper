//! Takes an ServiceNOW incident-export (as JSON) and maps it into a JSON that can be used to train
//! an OpenAI model.
//!
//! # Usage
//!
//! ```bash
//! $ snow_report_mapper path_to_input_file path_to_output_file
//! ```
//! ## Arguments:
//!
//! 1. Filepath to the ServiceNOW export
//! 2. Filepath to where the mapped report should be written to

use crate::loaders::servicenow;
use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub mod cli;
pub mod loaders;

/// Context for the errors that may occur during parsing
const ERR_MSG_UNABLE_TO_PROCESS_INPUT: &str = "Unable to process input file";

/// Represents an OpenAI model training entry
#[derive(Serialize, Deserialize, Debug)]
struct OutputEntry<'a> {
    /// Title of the ServiceNOW incident
    name: &'a str,
    /// Group to which the incident was assigned to
    assignment_group: &'a str,
}

fn main() -> Result<()> {
    let args: cli::Args = cli::parse();

    // Load and parse input-file
    let snow_report = servicenow::load_incidents(args.file_incidents)?;
    let result = map_data(&snow_report)?;
    write_result(&result, args.file_output)?;
    println!("Finished");
    Ok(())
}

/// Writes the mapped result to the output-file
///
/// # Parameters
///
/// 1. `result`: Mapped result
/// 2. `filename`: Output-filepath
fn write_result<'a>(result: &Vec<OutputEntry>, filename: String) -> Result<()> {
    let result_json_result = serde_json::to_string_pretty(&result)
        .with_context(|| format!("Failed to parse input file '{}'", &filename))?;
    std::fs::write(&filename, result_json_result)
        .with_context(|| format!("Failed to write to: '{}'", &filename))?;
    println!("{} entries written", &result.len());
    Ok(())
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
fn map_data(input_data: &Value) -> Result<Vec<OutputEntry>> {
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
