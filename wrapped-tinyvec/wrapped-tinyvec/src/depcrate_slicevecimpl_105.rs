// Generated macro for impl_105 (impl)
macro_rules! Depcrate_slicevecimpl_105 {
() => {
// Module: crate::slicevec
// Provides: {"impl_105"}
// Dependencies: {}
impl < 's , T , A > From < & 's mut A > for SliceVec < 's , T > where A : AsMut < [T] > , { # [doc = " Calls `AsRef::as_mut` then uses the full slice as the initial length."] # [doc = " ## Example"] # [doc = " ```rust"] # [doc = " # use tinyvec::*;"] # [doc = " let mut arr = [0, 0];"] # [doc = " let mut sv = SliceVec::from(&mut arr);"] # [doc = " ```"] # [inline] fn from (a : & 's mut A) -> Self { let data = a . as_mut () ; let len = data . len () ; Self { data , len } } }
};
}
