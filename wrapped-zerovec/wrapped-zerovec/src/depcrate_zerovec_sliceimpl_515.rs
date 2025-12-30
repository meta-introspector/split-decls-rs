// Generated macro for impl_515 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_515 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_515"}
// Dependencies: {}
impl < T > PartialEq < ZeroSlice < T > > for ZeroSlice < T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & ZeroSlice < T >) -> bool { self . as_zerovec () . eq (& other . as_zerovec ()) } }
};
}
