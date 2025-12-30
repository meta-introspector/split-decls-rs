// Generated macro for test_summary (function)
macro_rules! Depcrate_runnertest_summary {
() => {
// Module: crate::runner
// Provides: {"test_summary"}
// Dependencies: {}
# [doc = " Summary display at the end of the test run."] pub fn test_summary (passed : usize , failed : usize , ignored : usize) { println ! ("\ntest result: {} {} passed; {} failed; {} ignored" , if failed == 0 { "OK" } else { "FAILED" } , passed , failed , ignored) ; if failed != 0 { std :: process :: exit (101) ; } }
};
}
