// Generated macro for impl_3162 (impl)
macro_rules! Depcrate_sync_mpmc_wakerimpl_3162 {
() => {
// Module: crate::sync::mpmc::waker
// Provides: {"impl_3162"}
// Dependencies: {}
impl Drop for SyncWaker { # [inline] fn drop (& mut self) { debug_assert ! (self . is_empty . load (Ordering :: SeqCst)) ; } }
};
}
