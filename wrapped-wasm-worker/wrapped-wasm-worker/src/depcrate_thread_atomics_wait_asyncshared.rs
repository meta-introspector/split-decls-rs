// Generated macro for Shared (struct)
macro_rules! Depcrate_thread_atomics_wait_asyncShared {
() => {
// Module: crate::thread::atomics::wait_async
// Provides: {"Shared"}
// Dependencies: {}
# [doc = " Shared state for polyfill implementation."] # [derive (Debug)] struct Shared { # [doc = " [`true`] when finished."] finished : Cell < bool > , # [doc = " Stores [`Waker`] for callback."] waker : RefCell < Option < Waker > > , }
};
}
