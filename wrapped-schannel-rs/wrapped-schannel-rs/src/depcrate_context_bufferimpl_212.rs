// Generated macro for impl_212 (impl)
macro_rules! Depcrate_context_bufferimpl_212 {
() => {
// Module: crate::context_buffer
// Provides: {"impl_212"}
// Dependencies: {}
impl Drop for ContextBuffer { fn drop (& mut self) { unsafe { Identity :: FreeContextBuffer (self . 0 . pvBuffer) ; } } }
};
}
