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
use anyhow::Result;
use snow_report_mapper::{run, RunOptions};

pub mod cli;
pub mod loaders;
pub mod mappers;
pub mod serializers;
pub mod stats;
pub mod writers;

fn main() -> Result<()> {
    let args: cli::Args = cli::parse();

    run(
        &args.file_incidents,
        &args.file_assignment_groups,
        &args.file_output,
        RunOptions {
            verbose: args.verbose,
            print_stats: args.stats,
        },
    )
}
