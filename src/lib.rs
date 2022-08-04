use crate::loaders::servicenow;
use crate::stats::Stats;
use anyhow::Context;

mod loaders;
mod mappers;
mod serializers;
mod stats;
mod writers;

pub struct Options {
    pub verbose: bool,
    pub print_stats: bool,
}

/// Load and processes the raw input data and writes the processed output.
///
/// Following steps are done during the process:
///
/// - Load and parse incidents export
/// - Load and parse assignment groups
pub fn run(
    file_incidents: &String,
    file_assignment_groups: &String,
    file_output: &String,
    options: Options,
) -> anyhow::Result<()> {
    let mut stats = Stats::new();
    // Load and parse incidents
    let incidents_raw = std::fs::read_to_string(&file_incidents)
        .with_context(|| format!("Can't read {}", &file_incidents))?;
    if options.verbose {
        println!("Parsing incidents");
    }
    let snow_report = servicenow::parse_incidents(&incidents_raw)?;
    if options.verbose {
        println!("{} incidents found", snow_report.len());
    }
    let incidents_deduped = servicenow::deduped_incidents(snow_report);
    if options.verbose {
        println!("{} incidents left after de-duping", incidents_deduped.len());
    }
    // Load, parse and map assignment groups
    let assignment_groups_raw = std::fs::read_to_string(&file_assignment_groups)
        .with_context(|| format!("Can't read {}", &file_incidents))?;
    if options.verbose {
        println!("Parsing assignment groups");
    }
    let mut assignment_groups = servicenow::parse_assignment_groups(&assignment_groups_raw)?;
    if options.verbose {
        println!("{} assignment groups found", assignment_groups.len());
        println!("Mapping data");
    }
    let assignment_groups_indices =
        mappers::servicenow::map_assignment_groups(&mut assignment_groups);
    // Map and write output data
    let result = mappers::output::map_data(
        &incidents_deduped,
        &assignment_groups_indices,
        &assignment_groups,
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
