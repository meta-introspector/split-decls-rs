// Generated macro for impl_379 (impl)
macro_rules! Depcrate_thread_scopeimpl_379 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_379"}
// Dependencies: {}
impl < T > Debug for ScopedJoinHandle < '_ , T > { fn fmt (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . debug_struct ("ScopedJoinHandle") . field ("handle" , & self . handle) . field ("_scope" , & self . _scope) . finish () } }
};
}
