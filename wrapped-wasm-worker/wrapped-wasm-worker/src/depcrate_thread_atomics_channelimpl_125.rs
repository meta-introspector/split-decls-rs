// Generated macro for impl_125 (impl)
macro_rules! Depcrate_thread_atomics_channelimpl_125 {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"impl_125"}
// Dependencies: {}
impl < T > Sender < T > { # [doc = " Send an `event` to the corresponding [`Receiver`]."] pub (super) fn send (& self , event : T) -> Result < () , SendError < T > > { self . inner . as_ref () . expect ("`inner` not found") . send (event) ? ; self . waker . wake () ; Ok (()) } }
};
}
