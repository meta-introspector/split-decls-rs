// Generated macro for drop_token_with_children (function)
macro_rules! Depcrate_sync_tests_loom_cancellation_tokendrop_token_with_children {
() => {
// Module: crate::sync::tests::loom_cancellation_token
// Provides: {"drop_token_with_children"}
// Dependencies: {}
# [ignore] # [test] fn drop_token_with_children () { loom :: model (| | { let token1 = CancellationToken :: new () ; let child_token1 = token1 . child_token () ; let child_token2 = token1 . child_token () ; let th1 = thread :: spawn (move | | { drop (token1) ; }) ; let th2 = thread :: spawn (move | | { drop (child_token1) ; }) ; let th3 = thread :: spawn (move | | { drop (child_token2) ; }) ; assert_ok ! (th1 . join ()) ; assert_ok ! (th2 . join ()) ; assert_ok ! (th3 . join ()) ; }) ; }
};
}
