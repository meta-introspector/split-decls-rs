// Generated macro for test_is_finished (function)
macro_rules! Depcrate_thread_teststest_is_finished {
() => {
// Module: crate::thread::tests
// Provides: {"test_is_finished"}
// Dependencies: {}
# [test] fn test_is_finished () { let b = Arc :: new (Barrier :: new (2)) ; let t = thread :: spawn ({ let b = b . clone () ; move | | { b . wait () ; 1234 } }) ; assert_eq ! (t . is_finished () , false) ; b . wait () ; let start = Instant :: now () ; while ! t . is_finished () { assert ! (start . elapsed () < Duration :: from_secs (2)) ; thread :: sleep (Duration :: from_millis (15)) ; } let join_time = Instant :: now () ; assert_eq ! (t . join () . unwrap () , 1234) ; assert ! (join_time . elapsed () < Duration :: from_secs (2)) ; }
};
}
