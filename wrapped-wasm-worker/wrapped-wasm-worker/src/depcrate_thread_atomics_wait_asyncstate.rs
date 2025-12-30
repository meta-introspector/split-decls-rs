// Generated macro for State (enum)
macro_rules! Depcrate_thread_atomics_wait_asyncState {
() => {
// Module: crate::thread::atomics::wait_async
// Provides: {"State"}
// Dependencies: {}
# [doc = " State for [`WaitAsync`] [`Future`] implementation."] # [derive (Debug)] enum State { # [doc = " Atomic request was ready immediately."] Ready , # [doc = " [`Promise`](js_sys::Promise) returned by [`Atomics::wait_async()`]."] WaitAsync (JsFuture) , # [doc = " Polyfill implementation of [`Atomics::wait_async()`]."] Polyfill (Rc < Shared >) , }
};
}
