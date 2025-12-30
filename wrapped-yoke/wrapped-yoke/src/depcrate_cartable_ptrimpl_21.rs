// Generated macro for impl_21 (impl)
macro_rules! Depcrate_cartable_ptrimpl_21 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > CartablePointerLike for Box < T > { type Raw = T ; # [inline] fn into_raw (self) -> NonNull < T > { unsafe { NonNull :: new_unchecked (Box :: into_raw (self)) } } # [inline] unsafe fn drop_raw (pointer : NonNull < T >) { let _box = unsafe { Box :: from_raw (pointer . as_ptr ()) } ; # [cfg (test)] DROP_INVOCATIONS . with (| x | x . set (x . get () + 1)) } }
};
}
