use anyhow::{Context, Result};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct IncidentExport {
    pub records: Vec<Incident>,
}

#[derive(Deserialize)]
pub struct AssignmentGroupExport {
    pub result: Vec<AssignmentGroup>,
}

#[derive(Deserialize)]
pub struct AssignmentGroup {
    pub sys_id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Incident {
    /// Title of the ServiceNOW incident
    pub short_description: String,
    /// Group to which the incident was assigned to
    pub assignment_group: String,
}

pub fn parse_incidents(input_raw: &str) -> Result<Vec<Incident>> {
    println!("Parsing incidents");
    let incidents: IncidentExport =
        serde_json::from_str(input_raw).context("Unable to parse file")?;
    Ok(incidents.records)
}

pub fn parse_assignment_groups(input_raw: &str) -> Result<Vec<AssignmentGroup>> {
    println!("Parsing assignment groups");
    let incidents: AssignmentGroupExport =
        serde_json::from_str(input_raw).context("Unable to parse file")?;
    Ok(incidents.result)
}
