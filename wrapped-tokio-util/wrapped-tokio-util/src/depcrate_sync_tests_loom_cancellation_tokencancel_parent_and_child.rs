// Generated macro for cancel_parent_and_child (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokencancel_parent_and_child {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"cancel_parent_and_child"}
// Dependencies: {}
# [ignore] # [test] fn cancel_parent_and_child () { loom :: model (| | { let token1 = CancellationToken :: new () ; let token2 = token1 . clone () ; let child_token = token1 . child_token () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { token2 . cancel () ; }) ; let th3 = thread :: spawn (move | | { child_token . cancel () ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
};
}
