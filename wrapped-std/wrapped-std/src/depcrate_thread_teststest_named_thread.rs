// Generated macro for test_named_thread (function)
macro_rules! Depcrate_thread_teststest_named_thread {
() => {
// Module: crate::thread::tests
// Provides: {"test_named_thread"}
// Dependencies: {}
# [test] fn test_named_thread () { Builder :: new () . name ("ada lovelace" . to_string ()) . spawn (move | | { assert ! (thread :: current () . name () . unwrap () == "ada lovelace" . to_string ()) ; }) . unwrap () . join () . unwrap () ; }
};
}
