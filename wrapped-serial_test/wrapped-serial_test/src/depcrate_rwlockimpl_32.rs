// Generated macro for impl_32 (impl)
macro_rules! Depcrate_rwlockimpl_32 {
() => {
// Module: crate::rwlock
// Provides: {"impl_32"}
// Dependencies: {}
impl Drop for MutexGuardWrapper < '_ > { fn drop (& mut self) { # [cfg (feature = "logging")] debug ! ("End serial") ; self . locks . arc . condvar . notify_one () ; } }
};
}
