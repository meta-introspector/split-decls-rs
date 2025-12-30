// Generated macro for impl_797 (impl)
macro_rules! Depcrate_yoke_implsimpl_797 {
() => {
// Module: crate::yoke_impls
// Provides: {"impl_797"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `yoke` Cargo feature of the `zerovec` crate"] unsafe impl < 'a , T : 'static + AsULE > Yokeable < 'a > for ZeroVec < 'static , T > { type Output = ZeroVec < 'a , T > ; # [inline] fn transform (& 'a self) -> & 'a Self :: Output { self } # [inline] fn transform_owned (self) -> Self :: Output { self } # [inline] unsafe fn make (from : Self :: Output) -> Self { debug_assert ! (mem :: size_of ::< Self :: Output > () == mem :: size_of ::< Self > ()) ; let from = mem :: ManuallyDrop :: new (from) ; let ptr : * const Self = (& * from as * const Self :: Output) . cast () ; ptr :: read (ptr) } # [inline] fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { unsafe { f (mem :: transmute :: < & mut Self , & mut Self :: Output > (self)) } } }
};
}
