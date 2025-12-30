// Generated macro for test_thread_os_id_not_equal (function)
macro_rules! Depcrate_thread_teststest_thread_os_id_not_equal {
() => {
// Module: crate::thread::tests
// Provides: {"test_thread_os_id_not_equal"}
// Dependencies: {}
# [test] fn test_thread_os_id_not_equal () { let spawned_id = thread :: spawn (| | thread :: current_os_id ()) . join () . unwrap () ; let current_id = thread :: current_os_id () ; assert ! (current_id != spawned_id) ; }
};
}
