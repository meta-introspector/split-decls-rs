// Generated macro for is_main_thread (function)
macro_rules! Depcrate_thread_atomicsis_main_thread {
() => {
// Module: crate::thread::atomics
// Provides: {"is_main_thread"}
// Dependencies: {}
# [doc = " Determined if the current thread is the main thread. Make sure to"] # [doc = " call at least once on the main thread!"] pub (super) fn is_main_thread () -> bool { # [doc = " Saves the [`ThreadId`] of the main thread."] # [allow (clippy :: disallowed_methods , reason = "this will be called at least once from the main thread before being cached")] static MAIN_THREAD : LazyLock < ThreadId > = LazyLock :: new (current_id) ; * MAIN_THREAD == current_id () }
};
}
