// Generated macro for impl_42 (impl)
macro_rules! Depcrate_arcimpl_42 {
() => {
// Module: crate::arc
// Provides: {"impl_42"}
// Dependencies: {}
impl < T > Arc < [MaybeUninit < T >] > { # [doc = " Create an Arc contains an array `[MaybeUninit<T>]` of `len`."] pub fn new_uninit_slice (len : usize) -> Self { UniqueArc :: new_uninit_slice (len) . shareable () } # [doc = " Obtain a mutable slice to the stored `[MaybeUninit<T>]`."] # [deprecated (since = "0.1.8" , note = "this function previously was UB and now panics for non-unique `Arc`s. Use `UniqueArc` or `get_mut` instead.")] # [track_caller] pub fn as_mut_slice (& mut self) -> & mut [MaybeUninit < T >] { must_be_unique (self) } # [doc = " # Safety"] # [doc = ""] # [doc = " Must initialize all fields before calling this function."] # [inline] pub unsafe fn assume_init (self) -> Arc < [T] > { Arc :: from_raw_inner (ManuallyDrop :: new (self) . ptr () as _) } }
};
}
