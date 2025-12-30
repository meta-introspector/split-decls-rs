// Generated macro for drop_and_cancel_token (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokendrop_and_cancel_token {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"drop_and_cancel_token"}
// Dependencies: {}
# [ignore] # [test] fn drop_and_cancel_token () { loom :: model (| | { let token1 = CancellationToken :: new () ; let token2 = token1 . clone () ; let child_token = token1 . child_token () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { token2 . cancel () ; }) ; let th3 = thread :: spawn (move | | { drop (child_token) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
};
}
