// Generated macro for impl_126 (impl)
macro_rules! Depcrate_thread_atomics_channelimpl_126 {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"impl_126"}
// Dependencies: {}
impl < T > Clone for Sender < T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , waker : Arc :: clone (& self . waker) , } } }
};
}
