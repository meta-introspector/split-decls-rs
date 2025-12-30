// Generated macro for impl_220 (impl)
macro_rules! Depcrate_unique_arcimpl_220 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_220"}
// Dependencies: {}
# [cfg (feature = "unsize")] unsafe impl < T , U : ? Sized > unsize :: CoerciblePtr < U > for UniqueArc < T > { type Pointee = T ; type Output = UniqueArc < U > ; fn as_sized_ptr (& mut self) -> * mut T { unsize :: CoerciblePtr :: < U > :: as_sized_ptr (& mut self . 0) } unsafe fn replace_ptr (self , new : * mut U) -> UniqueArc < U > { let inner = ManuallyDrop :: new (self) ; UniqueArc (ptr :: read (& inner . 0) . replace_ptr (new)) } }
};
}
