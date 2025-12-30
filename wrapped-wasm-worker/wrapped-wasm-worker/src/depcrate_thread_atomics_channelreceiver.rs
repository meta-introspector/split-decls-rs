// Generated macro for Receiver (struct)
macro_rules! Depcrate_thread_atomics_channelReceiver {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " Async version of [`mpsc::Receiver`]."] pub (super) struct Receiver < T > { # [doc = " Actual [`mpsc::Sender`]."] receiver : mpsc :: Receiver < T > , # [doc = " Shared [`Waker`](std::task::Waker) between [`Receiver`] and [`Sender`]."] waker : Arc < AtomicWaker > , }
};
}
