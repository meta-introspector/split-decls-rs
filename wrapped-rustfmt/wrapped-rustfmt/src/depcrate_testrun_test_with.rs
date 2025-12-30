// Generated macro for run_test_with (function)
macro_rules! Depcrate_testrun_test_with {
() => {
// Module: crate::test
// Provides: {"run_test_with"}
// Dependencies: {}
fn run_test_with < F > (test_setting : & TestSetting , f : F) where F : FnOnce () , F : Send + 'static , { thread :: Builder :: new () . stack_size (test_setting . stack_size) . spawn (f) . expect ("Failed to create a test thread") . join () . expect ("Failed to join a test thread") }
};
}
