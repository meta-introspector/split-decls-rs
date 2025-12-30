// Generated macro for impl_213 (impl)
macro_rules! Depcrate_unique_arcimpl_213 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_213"}
// Dependencies: {}
impl < T > UniqueArc < MaybeUninit < T > > { # [doc = " Calls `MaybeUninit::write` on the contained value."] pub fn write (& mut self , val : T) -> & mut T { unsafe { let ptr = self . as_mut_ptr () as * mut T ; ptr . write (val) ; & mut * ptr } } # [doc = " Obtain a mutable pointer to the stored `MaybeUninit<T>`."] pub fn as_mut_ptr (& mut self) -> * mut MaybeUninit < T > { unsafe { & mut (* self . 0 . ptr ()) . data } } # [doc = " Convert to an initialized Arc."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is equivalent to `MaybeUninit::assume_init` and has the"] # [doc = " same safety requirements. You are responsible for ensuring that the `T`"] # [doc = " has actually been initialized before calling this method."] # [inline] pub unsafe fn assume_init (this : Self) -> UniqueArc < T > { UniqueArc (Arc { p : ManuallyDrop :: new (this) . 0 . p . cast () , phantom : PhantomData , }) } }
};
}
