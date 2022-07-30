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
use anyhow::Result;

pub mod cli;
pub mod loaders;
pub mod mappers;
pub mod writers;

fn main() -> Result<()> {
    let args: cli::Args = cli::parse();

    // Load and parse input-file
    let snow_report = servicenow::load_incidents(args.file_incidents)?;
    let result = mappers::output::map_data(&snow_report)?;
    writers::output::write_result(&result, args.file_output)?;
    println!("Finished");
    Ok(())
}
