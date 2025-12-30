// Generated macro for impl_127 (impl)
macro_rules! Depcrate_thread_atomics_channelimpl_127 {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"impl_127"}
// Dependencies: {}
impl < T > Drop for Sender < T > { fn drop (& mut self) { if Arc :: into_inner (self . inner . take () . expect ("`inner` not found")) . is_some () { self . waker . wake () ; } } }
};
}
