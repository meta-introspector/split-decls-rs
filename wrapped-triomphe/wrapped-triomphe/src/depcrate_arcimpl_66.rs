// Generated macro for impl_66 (impl)
macro_rules! Depcrate_arcimpl_66 {
() => {
// Module: crate::arc
// Provides: {"impl_66"}
// Dependencies: {}
# [cfg (feature = "unsize")] unsafe impl < T , U : ? Sized > unsize :: CoerciblePtr < U > for Arc < T > { type Pointee = T ; type Output = Arc < U > ; fn as_sized_ptr (& mut self) -> * mut T { self . p . as_ptr () as * mut T } unsafe fn replace_ptr (self , new : * mut U) -> Arc < U > { let inner = ManuallyDrop :: new (self) ; let p = inner . p . as_ptr () as * mut T ; Arc :: from_raw_inner (p . replace_ptr (new) as * mut ArcInner < U >) } }
};
}
