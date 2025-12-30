// Generated macro for impl_57 (impl)
macro_rules! Depcrate_arrayvecimpl_57 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_57"}
// Dependencies: {}
impl < A : Array > IntoIterator for ArrayVec < A > { type Item = A :: Item ; type IntoIter = ArrayVecIterator < A > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { ArrayVecIterator { base : 0 , tail : self . len , data : self . data } } }
};
}
