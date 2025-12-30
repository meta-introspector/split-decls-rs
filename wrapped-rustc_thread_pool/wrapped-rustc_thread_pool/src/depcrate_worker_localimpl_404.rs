// Generated macro for impl_404 (impl)
macro_rules! Depcrate_worker_localimpl_404 {
() => {
// Module: crate::worker_local
// Provides: {"impl_404"}
// Dependencies: {}
# [doc = " We prevent concurrent access to the underlying value in the"] # [doc = " Deref impl, thus any values safe to send across threads can"] # [doc = " be used with WorkerLocal."] unsafe impl < T : Send > Sync for WorkerLocal < T > { }
};
}
