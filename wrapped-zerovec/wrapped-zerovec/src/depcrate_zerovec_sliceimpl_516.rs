// Generated macro for impl_516 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_516 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_516"}
// Dependencies: {}
impl < T > PartialEq < [T] > for ZeroSlice < T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & [T]) -> bool { self . iter () . eq (other . iter () . copied ()) } }
};
}
