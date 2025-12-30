// Generated macro for cancel_token_owned (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokencancel_token_owned {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"cancel_token_owned"}
// Dependencies: {}
# [test] fn cancel_token_owned () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let th1 = thread :: spawn (move | | { block_on (async { token1 . cancelled_owned () . await ; }) ; }) ; let th2 = thread :: spawn (move | | { token . cancel () ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; }) ; }
};
}
