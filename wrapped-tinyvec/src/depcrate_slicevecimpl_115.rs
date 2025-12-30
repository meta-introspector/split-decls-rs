// Generated macro for impl_115 (impl)
macro_rules! Depcrate_slicevecimpl_115 {
() => {
// Module: crate::slicevec
// Provides: {"impl_115"}
// Dependencies: {}
impl < 's , T > IntoIterator for SliceVec < 's , T > { type Item = & 's mut T ; type IntoIter = core :: slice :: IterMut < 's , T > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . data . iter_mut () } }
};
}
