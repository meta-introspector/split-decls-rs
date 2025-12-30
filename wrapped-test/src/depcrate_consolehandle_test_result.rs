// Generated macro for handle_test_result (function)
macro_rules! Depcrate_consolehandle_test_result {
() => {
// Module: crate::console
// Provides: {"handle_test_result"}
// Dependencies: {}
fn handle_test_result (st : & mut ConsoleTestState , completed_test : CompletedTest) { let test = completed_test . desc ; let stdout = completed_test . stdout ; match completed_test . result { TestResult :: TrOk => { st . passed += 1 ; st . not_failures . push ((test , stdout)) ; } TestResult :: TrIgnored => { st . ignored += 1 ; st . ignores . push ((test , stdout)) ; } TestResult :: TrBench (bs) => { st . metrics . insert_metric (test . name . as_slice () , bs . ns_iter_summ . median , bs . ns_iter_summ . max - bs . ns_iter_summ . min ,) ; st . measured += 1 } TestResult :: TrFailed => { st . failed += 1 ; st . failures . push ((test , stdout)) ; } TestResult :: TrFailedMsg (msg) => { st . failed += 1 ; let mut stdout = stdout ; stdout . extend_from_slice (format ! ("note: {msg}") . as_bytes ()) ; st . failures . push ((test , stdout)) ; } TestResult :: TrTimedFail => { st . failed += 1 ; st . time_failures . push ((test , stdout)) ; } } }
};
}
