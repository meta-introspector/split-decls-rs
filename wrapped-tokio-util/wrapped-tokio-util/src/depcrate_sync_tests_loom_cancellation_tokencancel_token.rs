// Generated macro for cancel_token (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokencancel_token {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"cancel_token"}
// Dependencies: {}
# [test] fn cancel_token () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let th1 = thread :: spawn (move | | { block_on (async { token1 . cancelled () . await ; }) ; }) ; let th2 = thread :: spawn (move | | { token . cancel () ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; }) ; }
};
}
