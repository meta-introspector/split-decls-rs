// Generated macro for impl_518 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_518 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'a , T > PartialEq < ZeroSlice < T > > for ZeroVec < 'a , T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & ZeroSlice < T >) -> bool { self . eq (& other . as_zerovec ()) } }
};
}
