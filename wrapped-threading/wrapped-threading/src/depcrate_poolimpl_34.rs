// Generated macro for impl_34 (impl)
macro_rules! Depcrate_poolimpl_34 {
() => {
// Module: crate::pool
// Provides: {"impl_34"}
// Dependencies: {}
impl Drop for Pool { fn drop (& mut self) { self . join () ; unsafe { CloseThreadpoolCleanupGroup (self . 0 . CleanupGroup) ; CloseThreadpool (self . 0 . Pool) ; } } }
};
}
