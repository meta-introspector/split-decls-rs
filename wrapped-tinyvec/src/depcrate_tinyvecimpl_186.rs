// Generated macro for impl_186 (impl)
macro_rules! Depcrate_tinyvecimpl_186 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_186"}
// Dependencies: {}
impl < A : Array > IntoIterator for TinyVec < A > { type Item = A :: Item ; type IntoIter = TinyVecIterator < A > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { match self { TinyVec :: Inline (a) => TinyVecIterator :: Inline (a . into_iter ()) , TinyVec :: Heap (v) => TinyVecIterator :: Heap (v . into_iter ()) , } } }
};
}
