use crate::loaders::servicenow::Incident;
use crate::servicenow::AssignmentGroup;
use crate::Stats;
use anyhow::bail;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an OpenAI model training entry
///
/// # See also
///
/// - https://beta.openai.com/docs/guides/fine-tuning/data-formatting
/// - https://beta.openai.com/docs/guides/fine-tuning/preparing-your-dataset
#[derive(Serialize, Deserialize, Debug)]
pub struct FineTuningEntry {
    /// Title of the ServiceNOW incident
    prompt: String,
    /// Group to which the incident was assigned to
    completion: String,
}

/// Maps the ServiceNOW report data to OpenAI training data
///
/// # Arguments
///
/// - `incidents`: Parsed ServiceNow incidents
/// - `assignment_groups_indices`: Mapping between internal SNOW assignment group and processable OpenAI ID
/// - `assignment_groups`: Parsed assignment groups
/// - `stats`: Struct to hold mapping statistics
///
/// # Returns
///
/// The mapped OpenAI training data
///
/// # Bails out when
///
/// - an incident has been assigned to an unknown assignment group
pub fn map_data<'a, 'b>(
    incidents: &'a Vec<Incident>,
    assignment_groups_indices: &'a HashMap<String, usize>,
    assignment_groups: &'a Vec<AssignmentGroup>,
    stats: &'b mut Stats<'a>,
) -> anyhow::Result<Vec<FineTuningEntry>> {
    let mut result: Vec<FineTuningEntry> = Vec::new();
    let lookup = create_assignment_group_lookup(assignment_groups);

    for entry in incidents {
        let idx_assignment_group = assignment_groups_indices.get(&entry.assignment_group);
        if idx_assignment_group.is_none() {
            bail!(format!(
                "Unknown assignment group {}",
                &entry.assignment_group
            ))
        }
        let group_name = get_group_name(entry, &lookup)?;
        stats.inc_distribution(&group_name);
        result.push(FineTuningEntry {
            // See: https://beta.openai.com/docs/guides/fine-tuning/data-formatting
            prompt: format!("{}\n\n###\n\n", &entry.short_description).clone(),
            // The completion should be a string with a leading space
            completion: format!(" {}", *idx_assignment_group.unwrap()).clone(),
        })
    }
    Ok(result)
}

/// Creates a mapping table between the assignment group ID and the name, for faster lookup
///
/// # Arguments
///
/// - `assignment_groups`: List of assignment groups for which the mapping shall be created
///
/// # Returns
///
/// A mapping between ID and name
fn create_assignment_group_lookup(
    assignment_groups: &Vec<AssignmentGroup>,
) -> HashMap<&String, &String> {
    let mut lookup = HashMap::new();
    for entry in assignment_groups {
        lookup.insert(&entry.sys_id, &entry.name);
    }
    lookup
}

/// Gets the name for the assigned group, of an incident
///
/// # Arguments
///
/// # Bails out when
///
/// - an incident is assigned a group that is not available in the lookup map
fn get_group_name<'a, 'b>(
    entry: &'a Incident,
    lookup: &'b HashMap<&'a String, &'a String>,
) -> anyhow::Result<&'a String> {
    let group_name_wrapped = lookup.get(&entry.assignment_group);
    let group_name = match group_name_wrapped.is_none() {
        true => bail!(format!(
            "Unknown assignment group {}",
            &entry.assignment_group
        )),
        false => *group_name_wrapped.unwrap(),
    };
    Ok(group_name)
}
