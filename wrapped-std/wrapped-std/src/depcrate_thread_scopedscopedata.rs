// Generated macro for ScopeData (struct)
macro_rules! Depcrate_thread_scopedScopeData {
() => {
// Module: crate::thread::scoped
// Provides: {"ScopeData"}
// Dependencies: {}
pub (super) struct ScopeData { num_running_threads : Atomic < usize > , a_thread_panicked : Atomic < bool > , main_thread : Thread , }
};
}
