// Generated macro for tests (module)
macro_rules! Depcrate_errorstests {
() => {
// Module: crate::errors
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_error_is_send_and_sync () { fn test_send_sync < T : Send + Sync > () { } test_send_sync :: < super :: Error > () ; } }
};
}
