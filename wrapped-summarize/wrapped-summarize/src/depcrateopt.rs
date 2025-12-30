// Generated macro for Opt (enum)
macro_rules! DepcrateOpt {
() => {
// Module: crate
// Provides: {"Opt"}
// Dependencies: {}
# [derive (Parser , Debug)] enum Opt { # [doc = " Processes a set of trace files with identical events and analyze variance"] # [command (name = "aggregate")] Aggregate (AggregateOpt) , # [command (name = "diff")] Diff (DiffOpt) , # [doc = " Processes trace files and produces a summary"] # [command (name = "summarize")] Summarize (SummarizeOpt) , }
};
}
