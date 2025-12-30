// Generated macro for Shared (struct)
macro_rules! Depcrate_thread_atomics_oneshotShared {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"Shared"}
// Dependencies: {}
# [doc = " Shared state between [`Sender`] and [`Receiver`]."] struct Shared < T > { # [doc = " [`Mutex`] holding the returned value."] value : Mutex < State < T > > , # [doc = " [`Condvar`] to wake up any thread waiting on the return value."] cvar : Condvar , # [doc = " Registered [`Waker`](std::task::Waker) to be notified when the thread is"] # [doc = " finished."] waker : AtomicWaker , }
};
}
