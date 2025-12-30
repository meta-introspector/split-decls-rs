// Generated macro for impl_120 (impl)
macro_rules! Depcrate_slicevecimpl_120 {
() => {
// Module: crate::slicevec
// Provides: {"impl_120"}
// Dependencies: {}
impl < 's , T > PartialEq < & [T] > for SliceVec < 's , T > where T : PartialEq , { # [inline] fn eq (& self , other : & & [T]) -> bool { self . as_slice () . eq (* other) } }
};
}
