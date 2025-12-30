// Generated macro for impl_47 (impl)
macro_rules! Depcrate_arcimpl_47 {
() => {
// Module: crate::arc
// Provides: {"impl_47"}
// Dependencies: {}
# [cfg (not (feature = "unstable_dropck_eyepatch"))] impl < T : ? Sized > Drop for Arc < T > { # [inline] fn drop (& mut self) { self . drop_inner () ; } }
};
}
