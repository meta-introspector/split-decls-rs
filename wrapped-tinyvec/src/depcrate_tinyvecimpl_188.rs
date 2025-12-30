// Generated macro for impl_188 (impl)
macro_rules! Depcrate_tinyvecimpl_188 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'a , A : Array > IntoIterator for & 'a TinyVec < A > { type Item = & 'a A :: Item ; type IntoIter = core :: slice :: Iter < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
