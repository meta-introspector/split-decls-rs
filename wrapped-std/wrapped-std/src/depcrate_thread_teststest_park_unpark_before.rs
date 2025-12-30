// Generated macro for test_park_unpark_before (function)
macro_rules! Depcrate_thread_teststest_park_unpark_before {
() => {
// Module: crate::thread::tests
// Provides: {"test_park_unpark_before"}
// Dependencies: {}
# [test] fn test_park_unpark_before () { for _ in 0 .. 10 { thread :: current () . unpark () ; thread :: park () ; } }
};
}
