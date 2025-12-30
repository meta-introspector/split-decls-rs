// Generated macro for impl_59 (impl)
macro_rules! Depcrate_arrayvecimpl_59 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , A : Array > IntoIterator for & 'a ArrayVec < A > { type Item = & 'a A :: Item ; type IntoIter = core :: slice :: Iter < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
