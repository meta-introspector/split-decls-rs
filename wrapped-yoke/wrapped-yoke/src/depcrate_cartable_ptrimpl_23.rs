// Generated macro for impl_23 (impl)
macro_rules! Depcrate_cartable_ptrimpl_23 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < T > CartablePointerLike for Rc < T > { type Raw = T ; # [inline] fn into_raw (self) -> NonNull < T > { unsafe { NonNull :: new_unchecked (Rc :: into_raw (self) as * mut T) } } # [inline] unsafe fn drop_raw (pointer : NonNull < T >) { let _rc = unsafe { Rc :: from_raw (pointer . as_ptr ()) } ; # [cfg (test)] if Rc :: strong_count (& _rc) == 1 { DROP_INVOCATIONS . with (| x | x . set (x . get () + 1)) } } }
};
}
