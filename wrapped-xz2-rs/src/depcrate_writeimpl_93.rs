// Generated macro for impl_93 (impl)
macro_rules! Depcrate_writeimpl_93 {
() => {
// Module: crate::write
// Provides: {"impl_93"}
// Dependencies: {}
impl < W : Write > Drop for XzEncoder < W > { fn drop (& mut self) { if self . obj . is_some () { let _ = self . try_finish () ; } } }
};
}
