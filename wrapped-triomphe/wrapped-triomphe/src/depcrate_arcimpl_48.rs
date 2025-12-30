// Generated macro for impl_48 (impl)
macro_rules! Depcrate_arcimpl_48 {
() => {
// Module: crate::arc
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "unstable_dropck_eyepatch")] unsafe impl < # [may_dangle] T : ? Sized > Drop for Arc < T > { # [inline] fn drop (& mut self) { self . drop_inner () ; } }
};
}
