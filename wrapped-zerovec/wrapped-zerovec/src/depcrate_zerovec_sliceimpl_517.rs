// Generated macro for impl_517 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_517 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_517"}
// Dependencies: {}
impl < 'a , T > PartialEq < ZeroVec < 'a , T > > for ZeroSlice < T > where T : AsULE + PartialEq , { # [inline] fn eq (& self , other : & ZeroVec < 'a , T >) -> bool { self . as_zerovec () . eq (other) } }
};
}
