// Generated macro for impl_116 (impl)
macro_rules! Depcrate_slicevecimpl_116 {
() => {
// Module: crate::slicevec
// Provides: {"impl_116"}
// Dependencies: {}
impl < 's , T > PartialEq for SliceVec < 's , T > where T : PartialEq , { # [inline] fn eq (& self , other : & Self) -> bool { self . as_slice () . eq (other . as_slice ()) } }
};
}
