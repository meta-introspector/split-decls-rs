// Generated macro for current_thread_id (function)
macro_rules! Depcrate_sync_mpmc_wakercurrent_thread_id {
() => {
// Module: crate::sync::mpmc::waker
// Provides: {"current_thread_id"}
// Dependencies: {}
# [doc = " Returns a unique id for the current thread."] # [inline] pub fn current_thread_id () -> usize { thread_local ! { static DUMMY : u8 = const { 0 } } DUMMY . with (| x | (x as * const u8) . addr ()) }
};
}
