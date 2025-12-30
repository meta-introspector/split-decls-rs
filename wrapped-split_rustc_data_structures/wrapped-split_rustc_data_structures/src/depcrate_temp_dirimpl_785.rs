// Generated macro for impl_785 (impl)
macro_rules! Depcrate_temp_dirimpl_785 {
() => {
// Module: crate::temp_dir
// Provides: {"impl_785"}
// Dependencies: {}
impl Drop for MaybeTempDir { fn drop (& mut self) { let dir = unsafe { ManuallyDrop :: take (& mut self . dir) } ; if self . keep { let _ = dir . keep () ; } } }
};
}
