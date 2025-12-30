// Generated macro for ScopeData (struct)
macro_rules! Depcrate_thread_atomicsScopeData {
() => {
// Module: crate::thread::atomics
// Provides: {"ScopeData"}
// Dependencies: {}
# [doc = " Shared data between [`Scope`] and scoped threads."] # [derive (Debug)] pub (super) struct ScopeData { # [doc = " Number of running threads."] threads : AtomicU64 , # [doc = " Handle to the spawning thread."] thread : Thread , # [doc = " [`Waker`](std::task::Waker) to wake up a waiting [`Scope`]."] waker : AtomicWaker , }
};
}
