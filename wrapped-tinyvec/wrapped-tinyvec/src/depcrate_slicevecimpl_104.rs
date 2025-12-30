// Generated macro for impl_104 (impl)
macro_rules! Depcrate_slicevecimpl_104 {
() => {
// Module: crate::slicevec
// Provides: {"impl_104"}
// Dependencies: {}
impl < 's , T > From < & 's mut [T] > for SliceVec < 's , T > { # [doc = " Uses the full slice as the initial length."] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut arr = [0_i32; 2];"] # [doc = " let mut sv = SliceVec::from(&mut arr[..]);"] # [doc = " ```"] # [inline] fn from (data : & 's mut [T]) -> Self { let len = data . len () ; Self { data , len } } }
};
}
