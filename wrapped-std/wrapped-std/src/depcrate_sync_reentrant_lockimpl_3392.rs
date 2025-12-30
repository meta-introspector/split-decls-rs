// Generated macro for impl_3392 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3392 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3392"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > Deref for ReentrantLockGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { & self . lock . data } }
};
}
