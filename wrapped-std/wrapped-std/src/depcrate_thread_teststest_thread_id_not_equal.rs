// Generated macro for test_thread_id_not_equal (function)
macro_rules! Depcrate_thread_teststest_thread_id_not_equal {
() => {
// Module: crate::thread::tests
// Provides: {"test_thread_id_not_equal"}
// Dependencies: {}
# [test] fn test_thread_id_not_equal () { let spawned_id = thread :: spawn (| | thread :: current () . id ()) . join () . unwrap () ; assert ! (thread :: current () . id () != spawned_id) ; }
};
}
