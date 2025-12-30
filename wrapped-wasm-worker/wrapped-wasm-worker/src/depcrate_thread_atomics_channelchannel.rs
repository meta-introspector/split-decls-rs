// Generated macro for channel (function)
macro_rules! Depcrate_thread_atomics_channelchannel {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Async version of [`std::sync::mpsc`]."] pub (super) fn channel < T > () -> (Sender < T > , Receiver < T >) { let (sender , receiver) = mpsc :: channel () ; let waker = Arc :: new (AtomicWaker :: new ()) ; let sender = Sender { inner : Some (Arc :: new (sender)) , waker : Arc :: clone (& waker) , } ; let receiver = Receiver { receiver , waker } ; (sender , receiver) }
};
}
