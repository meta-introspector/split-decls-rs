// Generated macro for impl_18 (impl)
macro_rules! Depcrate_cartable_ptrimpl_18 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_18"}
// Dependencies: {}
unsafe impl < 'a , T > CartablePointerLike for & 'a T { type Raw = T ; # [inline] fn into_raw (self) -> NonNull < T > { self . into () } # [inline] unsafe fn drop_raw (_pointer : NonNull < T >) { } }
};
}
