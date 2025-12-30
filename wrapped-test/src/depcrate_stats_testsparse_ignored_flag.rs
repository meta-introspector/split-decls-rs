// Generated macro for parse_ignored_flag (function)
macro_rules! Depcrate_stats_testsparse_ignored_flag {
() => {
// Module: crate::stats::tests
// Provides: {"parse_ignored_flag"}
// Dependencies: {}
# [test] fn parse_ignored_flag () { let args = vec ! ["progname" . to_string () , "filter" . to_string () , "--ignored" . to_string ()] ; let opts = parse_opts (& args) . unwrap () . unwrap () ; assert_eq ! (opts . run_ignored , RunIgnored :: Only) ; }
};
}
