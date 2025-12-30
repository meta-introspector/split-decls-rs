// Generated macro for test_run_basic (function)
macro_rules! Depcrate_thread_teststest_run_basic {
() => {
// Module: crate::thread::tests
// Provides: {"test_run_basic"}
// Dependencies: {}
# [test] fn test_run_basic () { let (tx , rx) = channel () ; thread :: spawn (move | | { tx . send (()) . unwrap () ; }) ; rx . recv () . unwrap () ; }
};
}
