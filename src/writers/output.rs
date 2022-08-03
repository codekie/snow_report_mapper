/// Creates files that can be used to fine tune OpenAI models
use crate::mappers::output::FineTuningEntry;
use anyhow::{Context, Result};

/// Writes the mapped result to the output-file
///
/// # Arguments
///
/// - `result`: Mapped result
/// - `filename`: Output-filepath
///
/// # Returns
///
/// Amount of entries that were written
///
/// # Bails out when
///
/// - the result can't be deserialized
/// - the output-file can't be written
pub fn write_result<'a>(result: &'a Vec<FineTuningEntry>, filename: &'a String) -> Result<usize> {
    let result_json_result =
        serde_json::to_string_pretty(&result).context("Failed to deserialize result")?;
    std::fs::write(&filename, result_json_result)
        .with_context(|| format!("Failed to write to: '{}'", &filename))?;
    Ok(result.len())
}
