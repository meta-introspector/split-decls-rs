// Generated macro for impl_3394 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3394 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3394"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : fmt :: Display + ? Sized > fmt :: Display for ReentrantLockGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
