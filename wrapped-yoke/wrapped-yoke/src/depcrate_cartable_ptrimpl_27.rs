// Generated macro for impl_27 (impl)
macro_rules! Depcrate_cartable_ptrimpl_27 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_27"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > CloneableCartablePointerLike for Arc < T > { # [inline] unsafe fn addref_raw (pointer : NonNull < T >) { unsafe { Arc :: increment_strong_count (pointer . as_ptr ()) ; } } }
};
}
