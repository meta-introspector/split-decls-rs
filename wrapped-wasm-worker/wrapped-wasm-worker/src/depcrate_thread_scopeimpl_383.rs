// Generated macro for impl_383 (impl)
macro_rules! Depcrate_thread_scopeimpl_383 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_383"}
// Dependencies: {}
impl < F , T > Debug for ScopeFuture < '_ , '_ , F , T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_tuple ("ScopeFuture") . field (& self . 0) . finish () } }
};
}
