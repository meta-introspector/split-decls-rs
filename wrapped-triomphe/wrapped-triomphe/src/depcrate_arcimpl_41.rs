// Generated macro for impl_41 (impl)
macro_rules! Depcrate_arcimpl_41 {
() => {
// Module: crate::arc
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > Arc < MaybeUninit < T > > { # [doc = " Create an Arc contains an `MaybeUninit<T>`."] pub fn new_uninit () -> Self { Arc :: new (MaybeUninit :: < T > :: uninit ()) } # [doc = " Calls `MaybeUninit::write` on the value contained."] # [doc = ""] # [doc = " ## Panics"] # [doc = ""] # [doc = " If the `Arc` is not unique."] # [deprecated (since = "0.1.7" , note = "this function previously was UB and now panics for non-unique `Arc`s. Use `UniqueArc::write` instead.")] # [track_caller] pub fn write (& mut self , val : T) -> & mut T { UniqueArc :: write (must_be_unique (self) , val) } # [doc = " Obtain a mutable pointer to the stored `MaybeUninit<T>`."] pub fn as_mut_ptr (& mut self) -> * mut MaybeUninit < T > { unsafe { & mut (* self . ptr ()) . data } } # [doc = " # Safety"] # [doc = ""] # [doc = " Must initialize all fields before calling this function."] # [inline] pub unsafe fn assume_init (self) -> Arc < T > { Arc :: from_raw_inner (ManuallyDrop :: new (self) . ptr () . cast ()) } }
};
}
