// Generated macro for impl_16 (impl)
macro_rules! Depcrate_uniximpl_16 {
() => {
// Module: crate::unix
// Provides: {"impl_16"}
// Dependencies: {}
impl Drop for Handle { fn drop (& mut self) { if self . is_std { let _ = self . file . take () . unwrap () . into_raw_fd () ; } } }
};
}
