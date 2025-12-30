// Generated macro for impl_28 (impl)
macro_rules! Depcrate_streamimpl_28 {
() => {
// Module: crate::stream
// Provides: {"impl_28"}
// Dependencies: {}
impl Check { # [doc = " Test if this check is supported in this build of liblzma."] pub fn is_supported (& self) -> bool { let ret = unsafe { lzma_sys :: lzma_check_is_supported (* self as lzma_sys :: lzma_check) } ; ret != 0 } }
};
}
