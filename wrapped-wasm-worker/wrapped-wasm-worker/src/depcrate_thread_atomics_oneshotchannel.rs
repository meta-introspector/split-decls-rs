// Generated macro for channel (function)
macro_rules! Depcrate_thread_atomics_oneshotchannel {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates the oneshot channel."] pub (super) fn channel < T > () -> (Sender < T > , Receiver < T >) { let shared = Arc :: new (Shared { value : Mutex :: new (State :: Waiting) , cvar : Condvar :: new () , waker : AtomicWaker :: new () , }) ; (Sender (Some (Arc :: downgrade (& shared))) , Receiver (Some (shared)) ,) }
};
}
