use clap::crate_version;
use clap::Parser;

/// CLI arguments
#[derive(Parser, Debug)]
#[clap(name = "ServiceNOW report mapper")]
#[clap(version = crate_version!())]
#[clap(propagate_version = true)]
#[clap(about = "Prepares OpenAI refinement data, based on ServiceNOW incidents")]
#[clap(long_about = None)]
pub struct Args {
    /// Filepath to the SNOW incidents export
    #[clap(value_parser)]
    pub file_incidents: String,
    /// Filepath to the SNOW export of the assignment groups
    #[clap(value_parser)]
    pub file_assignment_groups: String,
    /// Filepath where the mapped training file should be stored to
    #[clap(value_parser)]
    pub file_output: String,
    /// Verbose output
    #[clap(short, long)]
    pub verbose: bool,
    /// Prints additional statistics
    #[clap(short, long)]
    pub stats: bool,
}

/// Parses the CLI arguments
///
/// # Returns
///
/// The parsed arguments as `Args` struct
pub fn parse() -> Args {
    Args::parse()
}
