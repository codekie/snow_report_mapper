use crate::loaders::servicenow::Incident;
use serde_derive::{Deserialize, Serialize};

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
/// 1. The parsed ServiceNOW incidents
///
/// # Returns
///
/// The mapped OpenAI training data
pub fn map_data(incidents: &Vec<Incident>) -> Vec<OutputEntry> {
    let mut result: Vec<OutputEntry> = Vec::new();
    println!("{} entries found", incidents.len());
    println!("Mapping data");
    for entry in incidents {
        result.push(OutputEntry {
            name: &entry.short_description,
            assignment_group: &entry.assignment_group,
        })
    }
    result
}
