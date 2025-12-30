// Generated macro for impl_92 (impl)
macro_rules! Depcrate_arc_swap_supportimpl_92 {
() => {
// Module: crate::arc_swap_support
// Provides: {"impl_92"}
// Dependencies: {}
unsafe impl < T > RefCnt for Arc < T > { type Base = T ; # [inline] fn into_ptr (me : Self) -> * mut Self :: Base { Arc :: into_raw (me) as * mut _ } # [inline] fn as_ptr (me : & Self) -> * mut Self :: Base { Arc :: as_ptr (me) as * mut _ } # [inline] unsafe fn from_ptr (ptr : * const Self :: Base) -> Self { Arc :: from_raw (ptr) } }
};
}
