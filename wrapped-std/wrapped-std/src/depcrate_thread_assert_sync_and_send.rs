// Generated macro for _assert_sync_and_send (function)
macro_rules! Depcrate_thread_assert_sync_and_send {
() => {
// Module: crate::thread
// Provides: {"_assert_sync_and_send"}
// Dependencies: {}
fn _assert_sync_and_send () { fn _assert_both < T : Send + Sync > () { } _assert_both :: < JoinHandle < () > > () ; _assert_both :: < Thread > () ; }
};
}
