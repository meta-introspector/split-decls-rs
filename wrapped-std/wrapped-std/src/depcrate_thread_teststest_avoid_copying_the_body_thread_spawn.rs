// Generated macro for test_avoid_copying_the_body_thread_spawn (function)
macro_rules! Depcrate_thread_teststest_avoid_copying_the_body_thread_spawn {
() => {
// Module: crate::thread::tests
// Provides: {"test_avoid_copying_the_body_thread_spawn"}
// Dependencies: {}
# [test] fn test_avoid_copying_the_body_thread_spawn () { avoid_copying_the_body (| f | { thread :: spawn (move | | { f () ; }) ; }) }
};
}
