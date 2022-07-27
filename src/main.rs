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
use anyhow::{Context, Result};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

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
    println!("Mapping SNOW data to OpenAI training data\n");
    // Get filenames from the command line
    let (input_filename, output_filename) = get_filenames()?;
    // Load and parse input-file
    let snow_report = load_and_parse(input_filename)?;
    let result = map_data(&snow_report)?;
    write_result(&result, output_filename)?;
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

/// Gets the file paths to the input file and to the output file from the command line arguments
///
/// # Returns
///
/// Tuple with:
///
/// 1. Path to input file (ServiceNOW export)
/// 2. Path to output file (OpenAI training data)
///
/// # Bails out when
///
/// - Input filename could not be found
/// - Output filename could not be found
fn get_filenames<'a>() -> Result<(String, String)> {
    let mut args = std::env::args();
    // First argument is the executable
    args.next();
    let mut input = args.next().context("Input filename is missing!")?;
    // If called with `cargo run`, then the first argument is the main-crate
    if input.ends_with("main.rs") {
        input = args.next().context("Input filename is missing!")?;
    }
    let output = args.next().context("Output filename is missing!")?;
    Ok((input, output))
}

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
fn load_and_parse(filename: String) -> Result<Value> {
    println!("Reading input file");
    let text =
        std::fs::read_to_string(&filename).with_context(|| format!("Can't read {}", &filename))?;
    Ok(serde_json::from_str(&text).with_context(|| format!("Unable to parse '{}'", &filename))?)
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
