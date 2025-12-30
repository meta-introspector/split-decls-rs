// Generated macro for impl_214 (impl)
macro_rules! Depcrate_unique_arcimpl_214 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_214"}
// Dependencies: {}
impl < T > UniqueArc < [MaybeUninit < T >] > { # [doc = " Create an Arc contains an array `[MaybeUninit<T>]` of `len`."] pub fn new_uninit_slice (len : usize) -> Self { let arc : Arc < HeaderSlice < () , [MaybeUninit < T >] > > = UniqueArc :: from_header_and_uninit_slice (() , len) . 0 ; let arc : Arc < [MaybeUninit < T >] > = arc . into () ; UniqueArc (arc) } # [doc = " # Safety"] # [doc = ""] # [doc = " Must initialize all fields before calling this function."] # [inline] pub unsafe fn assume_init_slice (Self (this) : Self) -> UniqueArc < [T] > { UniqueArc (this . assume_init ()) } }
};
}
