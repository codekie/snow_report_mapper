//! Takes an ServiceNOW incident-export (as JSON) and maps it into a JSON that can be used to train
//! an OpenAI model.
//!
//! # Usage
//!
//! ```
//! USAGE:
//!     snow_report_mapper [OPTIONS] <FILE_INCIDENTS> <FILE_ASSIGNMENT_GROUPS> <FILE_OUTPUT>
//!
//! ARGS:
//!     <FILE_INCIDENTS>            Filepath to the SNOW incidents export
//!     <FILE_ASSIGNMENT_GROUPS>    Filepath to the SNOW export of the assignment groups
//!     <FILE_OUTPUT>               Filepath where the mapped training file should be stored to
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -s, --stats      Prints additional statistics
//!     -v, --verbose    Verbose output
//!     -V, --version    Print version information
//! ```
//!
//! To get this help, run:
//!
//! ```bash
//! $ snow_report_mapper --help
//! ```
//!
//! ## Arguments:
//!
//! 1. Filepath to the ServiceNOW export
//! 2. Filepath to the assignment groups export
//! 3. Filepath to where the mapped report should be written to

use crate::loaders::servicenow;
use crate::stats::Stats;
use anyhow::{Context, Result};

pub mod cli;
pub mod loaders;
pub mod mappers;
pub mod serializers;
pub mod stats;
pub mod writers;

fn main() -> Result<()> {
    let mut stats = Stats::new();
    let args: cli::Args = cli::parse();
    // Load and parse incidents
    let incidents_raw = std::fs::read_to_string(&args.file_incidents)
        .with_context(|| format!("Can't read {}", &args.file_incidents))?;
    if args.verbose {
        println!("Parsing incidents");
    }
    let snow_report = servicenow::parse_incidents(&incidents_raw)?;
    if args.verbose {
        println!("{} incidents found", snow_report.len());
    }
    let incidents_deduped = servicenow::deduped_incidents(snow_report);
    if args.verbose {
        println!("{} incidents left after de-duping", incidents_deduped.len());
    }
    // Load, parse and map assignment groups
    let assignment_groups_raw = std::fs::read_to_string(&args.file_assignment_groups)
        .with_context(|| format!("Can't read {}", &args.file_incidents))?;
    if args.verbose {
        println!("Parsing assignment groups");
    }
    let mut assignment_groups = servicenow::parse_assignment_groups(&assignment_groups_raw)?;
    if args.verbose {
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
    let entries_written = writers::output::write_result(&result, &args.file_output)?;
    if args.verbose {
        println!("{} entries written", entries_written);
    }

    if args.stats {
        stats.print_stats();
    }
    Ok(())
}
