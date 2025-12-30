// Generated macro for impl_3395 (impl)
macro_rules! Depcrate_sync_reentrant_lockimpl_3395 {
() => {
// Module: crate::sync::reentrant_lock
// Provides: {"impl_3395"}
// Dependencies: {}
# [unstable (feature = "reentrant_lock" , issue = "121440")] impl < T : ? Sized > Drop for ReentrantLockGuard < '_ , T > { # [inline] fn drop (& mut self) { unsafe { * self . lock . lock_count . get () -= 1 ; if * self . lock . lock_count . get () == 0 { self . lock . owner . set (None) ; self . lock . mutex . unlock () ; } } } }
};
}
