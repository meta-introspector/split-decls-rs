// Generated macro for parse_show_output_flag (function)
macro_rules! Depcrate_testsparse_show_output_flag {
() => {
// Module: crate::tests
// Provides: {"parse_show_output_flag"}
// Dependencies: {}
# [test] fn parse_show_output_flag () { let args = vec ! ["progname" . to_string () , "filter" . to_string () , "--show-output" . to_string ()] ; let opts = parse_opts (& args) . unwrap () . unwrap () ; assert ! (opts . options . display_output) ; }
};
}
