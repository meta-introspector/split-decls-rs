// Generated macro for impl_3389 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3389 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3389"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Debug + ? Sized > fmt :: Debug for ReentrantLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("ReentrantLock") ; match self . try_lock () { Some (v) => d . field ("data" , & & * v) , None => d . field ("data" , & format_args ! ("<locked>")) , } ; d . finish_non_exhaustive () } }
};
}
