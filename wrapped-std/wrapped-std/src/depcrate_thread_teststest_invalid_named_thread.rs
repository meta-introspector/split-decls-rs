// Generated macro for test_invalid_named_thread (function)
macro_rules! Depcrate_thread_teststest_invalid_named_thread {
() => {
// Module: crate::thread::tests
// Provides: {"test_invalid_named_thread"}
// Dependencies: {}
# [test] # [should_panic] fn test_invalid_named_thread () { let _ = Builder :: new () . name ("ada l\0velace" . to_string ()) . spawn (| | { }) ; }
};
}
