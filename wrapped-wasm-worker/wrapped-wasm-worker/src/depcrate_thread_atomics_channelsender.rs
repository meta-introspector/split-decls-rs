// Generated macro for Sender (struct)
macro_rules! Depcrate_thread_atomics_channelSender {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " Async version of [`mpsc::Sender`]."] pub (super) struct Sender < T > { # [doc = " Actual [`mpsc::Sender`]."] inner : Option < Arc < mpsc :: Sender < T > > > , # [doc = " Shared [`Waker`](std::task::Waker) between [`Sender`] and [`Receiver`]."] waker : Arc < AtomicWaker > , }
};
}
