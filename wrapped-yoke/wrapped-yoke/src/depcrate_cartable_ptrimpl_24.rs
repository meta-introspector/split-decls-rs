// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cartable_ptrimpl_24 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > CloneableCartablePointerLike for Rc < T > { # [inline] unsafe fn addref_raw (pointer : NonNull < T >) { unsafe { Rc :: increment_strong_count (pointer . as_ptr ()) ; } } }
};
}
