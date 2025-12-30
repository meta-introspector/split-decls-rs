// Generated macro for impl_85 (impl)
macro_rules! Depcrate_arc_borrowimpl_85 {
() => {
// Module: crate::arc_borrow
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "unsize")] unsafe impl < 'lt , T : 'lt , U : ? Sized + 'lt > unsize :: CoerciblePtr < U > for ArcBorrow < 'lt , T > { type Pointee = T ; type Output = ArcBorrow < 'lt , U > ; fn as_sized_ptr (& mut self) -> * mut T { self . 0 . as_ptr () } unsafe fn replace_ptr (self , new : * mut U) -> ArcBorrow < 'lt , U > { let inner = ManuallyDrop :: new (self) ; ArcBorrow (inner . 0 . replace_ptr (new) , PhantomData) } }
};
}
