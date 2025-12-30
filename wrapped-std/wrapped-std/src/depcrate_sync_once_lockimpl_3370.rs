// Generated macro for impl_3370 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3370 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3370"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] unsafe impl < # [may_dangle] T > Drop for OnceLock < T > { # [inline] fn drop (& mut self) { if self . is_initialized () { unsafe { (& mut * self . value . get ()) . assume_init_drop () } ; } } }
};
}
