// Generated macro for impl_213 (impl)
macro_rules! Depcrate_context_bufferimpl_213 {
() => {
// Module: crate::context_buffer
// Provides: {"impl_213"}
// Dependencies: {}
impl Deref for ContextBuffer { type Target = [u8] ; fn deref (& self) -> & [u8] { if self . 0 . cbBuffer == 0 { return & [] ; } unsafe { slice :: from_raw_parts (self . 0 . pvBuffer as * const _ , self . 0 . cbBuffer as usize) } } }
};
}
