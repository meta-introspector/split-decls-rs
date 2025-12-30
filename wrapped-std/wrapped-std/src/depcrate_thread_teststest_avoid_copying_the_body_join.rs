// Generated macro for test_avoid_copying_the_body_join (function)
macro_rules! Depcrate_thread_teststest_avoid_copying_the_body_join {
() => {
// Module: crate::thread::tests
// Provides: {"test_avoid_copying_the_body_join"}
// Dependencies: {}
# [test] fn test_avoid_copying_the_body_join () { avoid_copying_the_body (| f | { let _ = thread :: spawn (move | | f ()) . join () ; }) }
};
}
