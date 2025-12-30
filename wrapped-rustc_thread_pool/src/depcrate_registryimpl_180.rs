// Generated macro for impl_180 (impl)
macro_rules! Depcrate_registryimpl_180 {
() => {
// Module: crate::registry
// Provides: {"impl_180"}
// Dependencies: {}
impl Drop for WorkerThread { fn drop (& mut self) { WORKER_THREAD_STATE . with (| t | { assert ! (t . get () . eq (& (self as * const _))) ; t . set (ptr :: null ()) ; }) ; } }
};
}
