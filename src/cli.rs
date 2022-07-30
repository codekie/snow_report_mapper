use clap::Parser;

/// CLI arguments
#[derive(Parser, Debug)]
#[clap(name = "ServiceNOW report mapper")]
#[clap(version = "0.1.0")]
#[clap(propagate_version = true)]
#[clap(about = "Prepares refinement data, based on ServiceNOW incidents")]
#[clap(long_about = None)]
pub struct Args {
    /// Filepath to the SNOW export of the assignment groups
    #[clap(short = 'a', long, value_parser)]
    pub file_assignment_groups: Option<String>,
    /// Filepath to the SNOW incidents export
    #[clap(value_parser)]
    pub file_incidents: String,
    /// Filepath where the mapped training file should be stored to
    #[clap(value_parser)]
    pub file_output: String,
}

/// Parses the CLI arguments
///
/// # Returns
///
/// The parsed arguments as `Args` struct
pub fn parse() -> Args {
    Args::parse()
}
