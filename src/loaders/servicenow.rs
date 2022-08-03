/// Loaders and structs for SNOW (ServiceNow) data exports
use crate::serializers::naive_datetime;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use serde_derive::Deserialize;
use std::collections::HashMap;

/// Represents an export of a list of incidents
#[derive(Deserialize)]
pub struct IncidentExport {
    /// List of incidents
    pub records: Vec<Incident>,
}

/// Represents an export of assignment groups
#[derive(Deserialize)]
pub struct AssignmentGroupExport {
    pub result: Vec<AssignmentGroup>,
}

/// A single SNOW assignment group
#[derive(Deserialize, Debug)]
pub struct AssignmentGroup {
    /// The internal ID an an assignment group within SNOW
    pub sys_id: String,
    /// The name of the assignment group
    pub name: String,
    /// The date when the assignment group was created
    #[serde(with = "naive_datetime")]
    pub sys_created_on: NaiveDateTime,
}

/// A single SNOW incident
#[derive(Deserialize, Clone)]
pub struct Incident {
    /// Title of the ServiceNOW incident
    pub short_description: String,
    /// Group (internal ID) to which the incident was assigned to
    pub assignment_group: String,
}

/// Parses an export of SNOW incidents and de-dupes entries (based on the title)
///
/// # Arguments
///
/// - `input_raw`: Raw file content
///
/// # Bails out when
///
/// - the de-duped content can't be deserialized to `Incident`s
pub fn parse_incidents(input_raw: &str) -> Result<Vec<Incident>> {
    let incidents: IncidentExport =
        serde_json::from_str(input_raw).context("Unable to parse file")?;
    Ok(incidents.records)
}

/// Parses an export of assignment groups
///
/// # Arguments
///
/// - `input_raw`: Raw file content
///
/// # Bails out when
///
/// - the content can't be deserialized to `AssignmentGroup`s
pub fn parse_assignment_groups(input_raw: &str) -> Result<Vec<AssignmentGroup>> {
    let incidents: AssignmentGroupExport =
        serde_json::from_str(input_raw).context("Unable to parse file")?;
    Ok(incidents.result)
}

/// De-dupes incidents with duplicate incident titles (to also prevent to have titles assigned
/// to different groups)
///
/// # Arguments
///
/// # Returns
pub fn deduped_incidents(incidents: Vec<Incident>) -> Vec<Incident> {
    // De-duping incidents with duplicate incident titles (to also prevent to have titles assigned
    // to different groups)
    let mut deduped_map: HashMap<String, Incident> = HashMap::new();
    for incident in incidents {
        deduped_map.insert(incident.short_description.clone(), incident);
    }
    let mut deduped_incidents: Vec<Incident> = Vec::new();
    for incident in deduped_map.values() {
        deduped_incidents.push((*incident).clone());
    }
    deduped_incidents
}
