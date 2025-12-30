// Generated macro for impl_114 (impl)
macro_rules! Depcrate_slicevecimpl_114 {
() => {
// Module: crate::slicevec
// Provides: {"impl_114"}
// Dependencies: {}
impl < 's , T > Extend < T > for SliceVec < 's , T > { # [inline] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for t in iter { self . push (t) } } }
};
}
