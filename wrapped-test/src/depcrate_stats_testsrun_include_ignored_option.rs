// Generated macro for run_include_ignored_option (function)
macro_rules! Depcrate_stats_testsrun_include_ignored_option {
() => {
// Module: crate::stats::tests
// Provides: {"run_include_ignored_option"}
// Dependencies: {}
# [test] fn run_include_ignored_option () { let mut opts = TestOpts :: new () ; opts . run_tests = true ; opts . run_ignored = RunIgnored :: Yes ; let tests = one_ignored_one_unignored_test () ; let filtered = filter_tests (& opts , tests) ; assert_eq ! (filtered . len () , 2) ; assert ! (! filtered [0] . desc . ignore) ; assert ! (! filtered [1] . desc . ignore) ; }
};
}
