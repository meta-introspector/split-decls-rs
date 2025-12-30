// Generated macro for impl_408 (impl)
macro_rules! Depcrate_worker_localimpl_408 {
() => {
// Module: crate::worker_local
// Provides: {"impl_408"}
// Dependencies: {}
impl < T > Deref for WorkerLocal < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { self . current () } }
};
}
