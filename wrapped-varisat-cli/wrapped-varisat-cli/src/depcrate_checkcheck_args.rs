// Generated macro for check_args (function)
macro_rules! Depcrate_checkcheck_args {
() => {
// Module: crate::check
// Provides: {"check_args"}
// Dependencies: {}
pub fn check_args () -> App < 'static , 'static > { SubCommand :: with_name ("--check") . arg_from_usage ("[INPUT] 'The input file to use (stdin if omitted)'") . arg_from_usage ("<proof-file> --proof=[FILE] 'The varisat proof file to check.'") . arg_from_usage ("[lrat-file] --write-lrat=[FILE] 'Convert the proof to LRAT.'") . arg_from_usage ("[clrat-file] --write-clrat=[FILE] 'Convert the proof to compressed (binary) LRAT.'" ,) }
};
}
