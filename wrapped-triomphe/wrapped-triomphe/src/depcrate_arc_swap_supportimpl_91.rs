// Generated macro for impl_91 (impl)
macro_rules! Depcrate_arc_swap_supportimpl_91 {
() => {
// Module: crate::arc_swap_support
// Provides: {"impl_91"}
// Dependencies: {}
unsafe impl < H , T > RefCnt for ThinArc < H , T > { type Base = c_void ; # [inline] fn into_ptr (me : Self) -> * mut Self :: Base { ThinArc :: into_raw (me) as * mut _ } # [inline] fn as_ptr (me : & Self) -> * mut Self :: Base { ThinArc :: as_ptr (me) as * mut _ } # [inline] unsafe fn from_ptr (ptr : * const Self :: Base) -> Self { ThinArc :: from_raw (ptr) } }
};
}
