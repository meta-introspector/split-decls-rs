// Generated macro for impl_133 (impl)
macro_rules! Depcrate_yokeableimpl_133 {
() => {
// Module: crate::yokeable
// Provides: {"impl_133"}
// Dependencies: {}
# [cfg (feature = "alloc")] unsafe impl < 'a , T : 'static + ToOwned + ? Sized > Yokeable < 'a > for Cow < 'static , T > where < T as ToOwned > :: Owned : Sized , { type Output = Cow < 'a , T > ; # [inline] fn transform (& 'a self) -> & 'a Cow < 'a , T > { self } # [inline] fn transform_owned (self) -> Cow < 'a , T > { self } # [inline] unsafe fn make (from : Cow < 'a , T >) -> Self { debug_assert ! (mem :: size_of ::< Cow <'a , T >> () == mem :: size_of ::< Self > ()) ; let ptr : * const Self = (& from as * const Self :: Output) . cast () ; let _ = core :: mem :: ManuallyDrop :: new (from) ; unsafe { core :: ptr :: read (ptr) } } # [inline] fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { unsafe { f (mem :: transmute :: < & 'a mut Self , & 'a mut Self :: Output > (self)) } } }
};
}
