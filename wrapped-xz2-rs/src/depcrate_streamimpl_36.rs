// Generated macro for impl_36 (impl)
macro_rules! Depcrate_streamimpl_36 {
() => {
// Module: crate::stream
// Provides: {"impl_36"}
// Dependencies: {}
impl Drop for Stream { fn drop (& mut self) { unsafe { lzma_sys :: lzma_end (& mut self . raw) ; } } }
};
}
