// Generated macro for impl_99 (impl)
macro_rules! Depcrate_writeimpl_99 {
() => {
// Module: crate::write
// Provides: {"impl_99"}
// Dependencies: {}
impl < W : Write > Drop for XzDecoder < W > { fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
};
}
