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

# Fine tune the model

Check the [official docs](https://beta.openai.com/docs/guides/fine-tuning) on how to fine-tune the model.

Before the generated files can be used, they have to be mapped to the `JSONL` format. For that it's recommended to use
the official [utility tools](https://beta.openai.com/docs/guides/fine-tuning/cli-data-preparation-tool):

```bash
$ openai tools fine_tunes.prepare_data -f <LOCAL_FILE>
```

It also shows suggestions if there are any and provides you with the proper command, to actual fine tune the model, in
the end.

## Troubleshooting

If you encounter an error message like:

```
The number of classes in file-7zfLf0xUhfTomlH7XPs5OGUL does not match the number of classes specified in the
hyperparameters
```

After triggering the fine tuning job, you can omit the parameters `--compute_classification_metrics` and
`--classification_n_classes` (additionally to its numeric value). You will lose the
[Classification specific metrics](https://beta.openai.com/docs/guides/fine-tuning/classification-specific-metrics)
but at least it works then.

# Motivation

The main reasons why I've built this, is to learn Rust and to get a bit more familiar with OpenAI. As a
"starter project" I've decided to train a model so that it's able to determine an appropriate assignment group for a
ServiceNow incident, based on the incident title. In order to that, I need training data in the appropriate format.
