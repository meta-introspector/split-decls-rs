// Generated macro for EmitMode (enum)
macro_rules! Depcrate_config_optionsEmitMode {
() => {
// Module: crate::config::options
// Provides: {"EmitMode"}
// Dependencies: {}
# [doc = " What Rustfmt should emit. Mostly corresponds to the `--emit` command line"] # [doc = " option."] # [config_type] pub enum EmitMode { # [doc = " Emits to files."] Files , # [doc = " Writes the output to stdout."] Stdout , # [doc = " Displays how much of the input file was processed"] Coverage , # [doc = " Unfancy stdout"] Checkstyle , # [doc = " Writes the resulting diffs in a JSON format. Returns an empty array"] # [doc = " `[]` if there were no diffs."] Json , # [doc = " Output the changed lines (for internal value only)"] ModifiedLines , # [doc = " Checks if a diff can be generated. If so, rustfmt outputs a diff and"] # [doc = " quits with exit code 1."] # [doc = " This option is designed to be run in CI where a non-zero exit signifies"] # [doc = " non-standard code formatting. Used for `--check`."] Diff , }
};
}
