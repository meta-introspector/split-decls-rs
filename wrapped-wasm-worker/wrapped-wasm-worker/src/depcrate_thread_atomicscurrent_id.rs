// Generated macro for current_id (function)
macro_rules! Depcrate_thread_atomicscurrent_id {
() => {
// Module: crate::thread::atomics
// Provides: {"current_id"}
// Dependencies: {}
# [doc = " Returns the [`ThreadId`] of the current thread without cloning the"] # [doc = " [`Arc`]."] fn current_id () -> ThreadId { THREAD . with (| cell | cell . get_or_init (Thread :: new) . id ()) }
};
}
