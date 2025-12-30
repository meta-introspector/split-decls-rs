// Generated macro for parse_include_ignored_flag (function)
macro_rules! Depcrate_testsparse_include_ignored_flag {
() => {
// Module: crate::tests
// Provides: {"parse_include_ignored_flag"}
// Dependencies: {}
# [test] fn parse_include_ignored_flag () { let args = vec ! ["progname" . to_string () , "filter" . to_string () , "--include-ignored" . to_string ()] ; let opts = parse_opts (& args) . unwrap () . unwrap () ; assert_eq ! (opts . run_ignored , RunIgnored :: Yes) ; }
};
}
