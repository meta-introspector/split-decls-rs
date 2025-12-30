// Generated macro for impl_187 (impl)
macro_rules! Depcrate_tinyvecimpl_187 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'a , A : Array > IntoIterator for & 'a mut TinyVec < A > { type Item = & 'a mut A :: Item ; type IntoIter = core :: slice :: IterMut < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
