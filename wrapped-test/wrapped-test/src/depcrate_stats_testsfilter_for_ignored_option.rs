// Generated macro for filter_for_ignored_option (function)
macro_rules! Depcrate_stats_testsfilter_for_ignored_option {
() => {
// Module: crate::stats::tests
// Provides: {"filter_for_ignored_option"}
// Dependencies: {}
# [test] fn filter_for_ignored_option () { let mut opts = TestOpts :: new () ; opts . run_tests = true ; opts . run_ignored = RunIgnored :: Only ; let tests = one_ignored_one_unignored_test () ; let filtered = filter_tests (& opts , tests) ; assert_eq ! (filtered . len () , 1) ; assert_eq ! (filtered [0] . desc . name . to_string () , "1") ; assert ! (! filtered [0] . desc . ignore) ; }
};
}
