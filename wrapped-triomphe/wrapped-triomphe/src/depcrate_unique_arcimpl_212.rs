// Generated macro for impl_212 (impl)
macro_rules! Depcrate_unique_arcimpl_212 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_212"}
// Dependencies: {}
impl < T : ? Sized > UniqueArc < T > { # [doc = " Convert to a shareable `Arc<T>` once we're done mutating it"] # [inline] pub fn shareable (self) -> Arc < T > { self . 0 } # [doc = " Creates a new [`UniqueArc`] from the given [`Arc`]."] # [doc = ""] # [doc = " An unchecked alternative to `Arc::try_unique()`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given `Arc` must have a reference count of exactly one"] # [doc = ""] pub (crate) unsafe fn from_arc (arc : Arc < T >) -> Self { debug_assert_eq ! (Arc :: count (& arc) , 1) ; Self (arc) } # [doc = " Creates a new `&mut `[`UniqueArc`] from the given `&mut `[`Arc`]."] # [doc = ""] # [doc = " An unchecked alternative to `Arc::try_as_unique()`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given `Arc` must have a reference count of exactly one"] pub (crate) unsafe fn from_arc_ref (arc : & mut Arc < T >) -> & mut Self { debug_assert_eq ! (Arc :: count (arc) , 1) ; & mut * (arc as * mut Arc < T > as * mut UniqueArc < T >) } }
};
}
