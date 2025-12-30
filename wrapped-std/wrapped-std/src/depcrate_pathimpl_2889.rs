// Generated macro for impl_2889 (impl)
macro_rules! Depcrate_pathimpl_2889 {
() => {
// Module: crate::path
// Provides: {"impl_2889"}
// Dependencies: {}
# [unstable (feature = "clone_to_uninit" , issue = "126799")] unsafe impl CloneToUninit for Path { # [inline] # [cfg_attr (debug_assertions , track_caller)] unsafe fn clone_to_uninit (& self , dst : * mut u8) { unsafe { self . inner . clone_to_uninit (dst) } } }
};
}
