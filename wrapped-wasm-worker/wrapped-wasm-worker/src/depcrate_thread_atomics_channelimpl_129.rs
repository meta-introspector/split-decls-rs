// Generated macro for impl_129 (impl)
macro_rules! Depcrate_thread_atomics_channelimpl_129 {
() => {
// Module: crate::thread::atomics::channel
// Provides: {"impl_129"}
// Dependencies: {}
impl < T > Debug for Receiver < T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("Receiver") . field ("receiver" , & self . receiver) . field ("waker" , & self . waker) . finish () } }
};
}
