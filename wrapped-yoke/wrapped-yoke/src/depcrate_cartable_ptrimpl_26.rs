// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cartable_ptrimpl_26 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > CartablePointerLike for Arc < T > { type Raw = T ; # [inline] fn into_raw (self) -> NonNull < T > { unsafe { NonNull :: new_unchecked (Arc :: into_raw (self) as * mut T) } } # [inline] unsafe fn drop_raw (pointer : NonNull < T >) { let _arc = unsafe { Arc :: from_raw (pointer . as_ptr ()) } ; # [cfg (test)] if Arc :: strong_count (& _arc) == 1 { DROP_INVOCATIONS . with (| x | x . set (x . get () + 1)) } } }
};
}
