// Generated macro for thread_id (function)
macro_rules! Depcratethread_id {
() => {
// Module: crate
// Provides: {"thread_id"}
// Dependencies: {}
# [doc = " The thread identifier of the calling thread."] pub fn thread_id () -> u32 { unsafe { GetCurrentThreadId () } }
};
}
