// Generated macro for impl_184 (impl)
macro_rules! Depcrate_thread_scopedimpl_184 {
() => {
// Module: crate::thread::scoped
// Provides: {"impl_184"}
// Dependencies: {}
# [stable (feature = "scoped_threads" , since = "1.63.0")] impl < 'scope , T > fmt :: Debug for ScopedJoinHandle < 'scope , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ScopedJoinHandle") . finish_non_exhaustive () } }
};
}
