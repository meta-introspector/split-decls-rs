// Generated macro for test (module)
macro_rules! Depcrate_fork_testtest {
() => {
// Module: crate::fork_test
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { rusty_fork_test ! { # [test] fn trivial () { } # [test] # [should_panic] fn panicking_child () { panic ! ("just testing a panic, nothing to see here") ; } # [test] # [should_panic] fn aborting_child () { :: std :: process :: abort () ; } } rusty_fork_test ! { #! [rusty_fork (timeout_ms = 1000)] # [test] # [cfg (feature = "timeout")] fn timeout_passes () { } # [test] # [should_panic] # [cfg (feature = "timeout")] fn timeout_fails () { println ! ("hello from child") ; :: std :: thread :: sleep (:: std :: time :: Duration :: from_millis (10000)) ; println ! ("goodbye from child") ; } } }
};
}
