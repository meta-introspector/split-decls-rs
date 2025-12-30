// Generated macro for mark_blocked (function)
macro_rules! Depcrate_registrymark_blocked {
() => {
// Module: crate::registry
// Provides: {"mark_blocked"}
// Dependencies: {}
# [doc = " Mark a Rayon worker thread as blocked. This triggers the deadlock handler"] # [doc = " if no other worker thread is active"] # [inline] pub fn mark_blocked () { let worker_thread = WorkerThread :: current () ; assert ! (! worker_thread . is_null ()) ; unsafe { let registry = & (* worker_thread) . registry ; registry . sleep . mark_blocked (& registry . deadlock_handler) } }
};
}
