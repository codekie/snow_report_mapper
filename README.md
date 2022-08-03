# ServiceNow report mapper

This crate creates a OpenAI fine-tuning file to enable the model to categorize ServiceNow incidents into assignment
groups.

# Usage

 ```
 USAGE:
     snow_report_mapper [OPTIONS] <FILE_INCIDENTS> <FILE_ASSIGNMENT_GROUPS> <FILE_OUTPUT>

 ARGS:
     <FILE_INCIDENTS>            Filepath to the SNOW incidents export
     <FILE_ASSIGNMENT_GROUPS>    Filepath to the SNOW export of the assignment groups
     <FILE_OUTPUT>               Filepath where the mapped training file should be stored to

 OPTIONS:
     -h, --help       Print help information
     -s, --stats      Prints additional statistics
     -v, --verbose    Verbose output
     -V, --version    Print version information
 ```

 To get this help, run:

 ```bash
 $ snow_report_mapper --help
 ```

For more details, check the rust documentation of this crate.

# Motivation

The main reasons why I've built this, is to learn Rust and to get a bit more familiar with OpenAI. As a
"starter project" I've decided to train a model so that it's able to determine an appropriate assignment group for a
ServiceNow incident, based on the incident title. In order to that, I need training data in the appropriate format.
