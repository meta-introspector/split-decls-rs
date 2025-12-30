// Generated macro for impl_306 (impl)
macro_rules! Depcrate_thread_atomicsimpl_306 {
() => {
// Module: crate::thread::atomics
// Provides: {"impl_306"}
// Dependencies: {}
impl < T > Debug for JoinHandle < T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("JoinHandle") . field ("receiver" , & self . receiver) . field ("thread" , & self . thread) . finish () } }
};
}
