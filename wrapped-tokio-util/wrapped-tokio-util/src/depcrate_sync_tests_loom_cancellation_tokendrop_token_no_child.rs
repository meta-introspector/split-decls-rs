// Generated macro for drop_token_no_child (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokendrop_token_no_child {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"drop_token_no_child"}
// Dependencies: {}
# [test] fn drop_token_no_child () { loom :: model (| | { let token = CancellationToken :: new () ; let token1 = token . clone () ; let token2 = token . clone () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { drop (token2) ; }) ; let th3 = thread :: spawn (move | | { drop (token) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
};
}
