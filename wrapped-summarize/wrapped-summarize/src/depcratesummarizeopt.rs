// Generated macro for SummarizeOpt (struct)
macro_rules! DepcrateSummarizeOpt {
() => {
// Module: crate
// Provides: {"SummarizeOpt"}
// Dependencies: {}
# [derive (Parser , Debug)] struct SummarizeOpt { file_prefix : PathBuf , # [doc = " Writes the analysis to a json file next to <file_prefix> instead of stdout"] # [arg (long = "json")] json : bool , # [doc = " Filter the output to items whose self-time is greater than this value"] # [arg (short = 'p' , long = "percent-above" , default_value = "0.0")] percent_above : f64 , }
};
}
