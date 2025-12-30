// Generated macro for impl_800 (impl)
macro_rules! Depcrate_yoke_implsimpl_800 {
() => {
// Module: crate::yoke_impls
// Provides: {"impl_800"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `yoke` Cargo feature of the `zerovec` crate"] # [cfg (feature = "alloc")] unsafe impl < 'a , K , V > Yokeable < 'a > for ZeroMap < 'static , K , V > where K : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , V : 'static + for < 'b > ZeroMapKV < 'b > + ? Sized , < K as ZeroMapKV < 'static > > :: Container : for < 'b > Yokeable < 'b > , < V as ZeroMapKV < 'static > > :: Container : for < 'b > Yokeable < 'b > , { type Output = ZeroMap < 'a , K , V > ; # [inline] fn transform (& 'a self) -> & 'a Self :: Output { unsafe { mem :: transmute :: < & Self , & Self :: Output > (self) } } # [inline] fn transform_owned (self) -> Self :: Output { debug_assert ! (mem :: size_of ::< Self :: Output > () == mem :: size_of ::< Self > ()) ; unsafe { let this = mem :: ManuallyDrop :: new (self) ; let ptr : * const Self :: Output = (& * this as * const Self) . cast () ; ptr :: read (ptr) } } # [inline] unsafe fn make (from : Self :: Output) -> Self { debug_assert ! (mem :: size_of ::< Self :: Output > () == mem :: size_of ::< Self > ()) ; let from = mem :: ManuallyDrop :: new (from) ; let ptr : * const Self = (& * from as * const Self :: Output) . cast () ; ptr :: read (ptr) } # [inline] fn transform_mut < F > (& 'a mut self , f : F) where F : 'static + for < 'b > FnOnce (& 'b mut Self :: Output) , { unsafe { f (mem :: transmute :: < & mut Self , & mut Self :: Output > (self)) } } }
};
}
