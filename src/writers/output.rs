use crate::mappers::output::OutputEntry;
use anyhow::{Context, Result};

/// Writes the mapped result to the output-file
///
/// # Parameters
///
/// 1. `result`: Mapped result
/// 2. `filename`: Output-filepath
pub fn write_result<'a>(result: &Vec<OutputEntry>, filename: String) -> Result<()> {
    let result_json_result = serde_json::to_string_pretty(&result)
        .with_context(|| format!("Failed to parse input file '{}'", &filename))?;
    std::fs::write(&filename, result_json_result)
        .with_context(|| format!("Failed to write to: '{}'", &filename))?;
    println!("{} entries written", &result.len());
    Ok(())
}
