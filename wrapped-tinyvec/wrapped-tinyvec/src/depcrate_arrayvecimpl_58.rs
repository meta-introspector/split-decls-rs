// Generated macro for impl_58 (impl)
macro_rules! Depcrate_arrayvecimpl_58 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , A : Array > IntoIterator for & 'a mut ArrayVec < A > { type Item = & 'a mut A :: Item ; type IntoIter = core :: slice :: IterMut < 'a , A :: Item > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
