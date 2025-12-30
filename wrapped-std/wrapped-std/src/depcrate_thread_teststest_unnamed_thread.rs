// Generated macro for test_unnamed_thread (function)
macro_rules! Depcrate_thread_teststest_unnamed_thread {
() => {
// Module: crate::thread::tests
// Provides: {"test_unnamed_thread"}
// Dependencies: {}
# [test] fn test_unnamed_thread () { thread :: spawn (move | | { assert ! (thread :: current () . name () . is_none ()) ; }) . join () . ok () . expect ("thread panicked") ; }
};
}
