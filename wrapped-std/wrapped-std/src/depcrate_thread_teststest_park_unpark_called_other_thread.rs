// Generated macro for test_park_unpark_called_other_thread (function)
macro_rules! Depcrate_thread_teststest_park_unpark_called_other_thread {
() => {
// Module: crate::thread::tests
// Provides: {"test_park_unpark_called_other_thread"}
// Dependencies: {}
# [test] fn test_park_unpark_called_other_thread () { for _ in 0 .. 10 { let th = thread :: current () ; let _guard = thread :: spawn (move | | { super :: sleep (Duration :: from_millis (50)) ; th . unpark () ; }) ; thread :: park () ; } }
};
}
