// Generated macro for impl_132 (impl)
macro_rules! Depcrate_sync_reusable_boximpl_132 {
() => {
// Module: crate::sync::reusable_box
// Provides: {"impl_132"}
// Dependencies: {}
impl < O , F : FnOnce () -> O > Drop for CallOnDrop < O , F > { fn drop (& mut self) { let f = unsafe { ManuallyDrop :: take (& mut self . f) } ; f () ; } }
};
}
