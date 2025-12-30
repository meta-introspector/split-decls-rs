// Generated macro for waits_until_zero (function)
macro_rules! Depcrate_serverwaits_until_zero {
() => {
// Module: crate::server
// Provides: {"waits_until_zero"}
// Dependencies: {}
# [test] fn waits_until_zero () { let (wait , _active) = WaitUntilZero :: new () ; assert_eq ! (wait . now_or_never () , None) ; let (wait , active) = WaitUntilZero :: new () ; let _active2 = active . clone () ; drop (active) ; assert_eq ! (wait . now_or_never () , None) ; let (wait , _) = WaitUntilZero :: new () ; assert_eq ! (wait . now_or_never () , Some (())) ; let (wait , active) = WaitUntilZero :: new () ; let active2 = active . clone () ; drop (active) ; drop (active2) ; assert_eq ! (wait . now_or_never () , Some (())) ; }
};
}
