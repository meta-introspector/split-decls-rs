// Generated macro for impl_3159 (impl)
macro_rules! Depcrate_sync_mpmc_wakerimpl_3159 {
() => {
// Module: crate::sync::mpmc::waker
// Provides: {"impl_3159"}
// Dependencies: {}
impl Drop for Waker { # [inline] fn drop (& mut self) { debug_assert_eq ! (self . selectors . len () , 0) ; debug_assert_eq ! (self . observers . len () , 0) ; } }
};
}
