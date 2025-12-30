// Generated macro for impl_653 (impl)
macro_rules! Depcrate_ffi_os_strimpl_653 {
() => {
// Module: crate::ffi::os_str
// Provides: {"impl_653"}
// Dependencies: {}
# [unstable (feature = "clone_to_uninit" , issue = "126799")] unsafe impl CloneToUninit for OsStr { # [inline] # [cfg_attr (debug_assertions , track_caller)] unsafe fn clone_to_uninit (& self , dst : * mut u8) { unsafe { self . inner . clone_to_uninit (dst) } } }
};
}
