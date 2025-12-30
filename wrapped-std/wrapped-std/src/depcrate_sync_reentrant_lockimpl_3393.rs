// Generated macro for impl_3393 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3393 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3393"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Debug + ? Sized > fmt :: Debug for ReentrantLockGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
