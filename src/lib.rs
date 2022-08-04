/// Contains the application logic for the ServiceNow mapper
use crate::loaders::servicenow;
use crate::servicenow::{AssignmentGroup, Incident};
use crate::stats::Stats;
use anyhow::Context;

mod loaders;
mod mappers;
mod serializers;
mod stats;
mod writers;

pub struct RunOptions {
    /// Print additional infos
    pub verbose: bool,
    /// Print statistics, collected during processing
    pub print_stats: bool,
    /// Trim the amount of incidents per assigment group by the given amount
    pub trim: Option<usize>,
}

/// Load and processes the raw input data and writes the processed output.
///
/// Following steps are done during the process:
///
/// - Load and parse incidents export
/// - De-dupe incidents by title
/// - Load and parse assignment groups
/// - Map input data to fine-tuning entries for OpenAI
/// - Write the output file
///
/// # Arguments
///
/// - `file_incidents`: Filepath to the incident export
/// - `file_assignment_groups`: Filepath to the assignment groups export
/// - `file_output`: Filepath to where the result has to be written to
/// - `options`: [Options][RunOptions]
///
/// # Bails out when
///
/// - the incident export can't be loaded or parsed
/// - the assignment groups can't be loaded or parsed
/// - an incident has been assigned to an unknown assignment group
/// - the result can't be deserialized
/// - the output-file can't be written
pub fn run(
    file_incidents: &String,
    file_assignment_groups: &String,
    file_output: &String,
    options: RunOptions,
) -> anyhow::Result<()> {
    let mut stats = Stats::new();
    let snow_report = load_and_parse_incidents(file_incidents, &options)?;
    let incidents_deduped = servicenow::deduped_incidents(snow_report);
    if options.verbose {
        println!("{} incidents left after de-duping", incidents_deduped.len());
    }
    let mut assignment_groups: Vec<AssignmentGroup> =
        load_and_parse_assignment_groups(file_assignment_groups, &options)?;
    // Create map between assignment group ID and arbitrary category ID for OpenAI
    let assignment_groups_indices =
        mappers::servicenow::map_assignment_groups(&mut assignment_groups);
    // Map data to fine-tuning entries for OpenAI
    let result = mappers::output::map_data(
        &incidents_deduped,
        &assignment_groups_indices,
        &assignment_groups,
        &options.trim,
        &mut stats,
    )
    .context("An error has occured during creating the mapping")?;

    let entries_written = writers::output::write_result(&result, file_output)?;
    if options.verbose {
        println!("{} entries written", entries_written);
    }

    if options.print_stats {
        stats.print_stats();
    }
    Ok(())
}

/// Loads and parses an incident export
///
/// # Arguments
///
/// - `file_incidents`: Filepath to the incident export
/// - `options`: [Options][RunOptions]
///
/// # Returns
///
/// The parsed incidents
///
/// # Bails out when
///
/// - the incident export can't be loaded or parsed
fn load_and_parse_incidents(
    file_incidents: &String,
    options: &RunOptions,
) -> anyhow::Result<Vec<Incident>> {
    let incidents_raw = std::fs::read_to_string(&file_incidents)
        .with_context(|| format!("Can't read {}", &file_incidents))?;
    if options.verbose {
        println!("Parsing incidents");
    }
    let snow_report = servicenow::parse_incidents(&incidents_raw)?;
    if options.verbose {
        println!("{} incidents found", snow_report.len());
    }
    Ok(snow_report)
}

/// Loads and parses the assignment groups export
///
/// # Arguments
///
/// - `file_assignment_groups`: Filepath to the assignment groups export
/// - `options`: [Options][RunOptions]
///
/// # Returns
///
/// The parsed assignment groups
///
/// # Bails out when
///
/// - the assignment groups can't be loaded or parsed
fn load_and_parse_assignment_groups(
    file_assignment_groups: &String,
    options: &RunOptions,
) -> anyhow::Result<Vec<AssignmentGroup>> {
    let assignment_groups_raw = std::fs::read_to_string(&file_assignment_groups)
        .with_context(|| format!("Can't read {}", &file_assignment_groups))?;
    if options.verbose {
        println!("Parsing assignment groups");
    }
    let assignment_groups: Vec<AssignmentGroup> =
        servicenow::parse_assignment_groups(&assignment_groups_raw)?;
    if options.verbose {
        println!("{} assignment groups found", assignment_groups.len());
        println!("Mapping data");
    }
    Ok(assignment_groups)
}
