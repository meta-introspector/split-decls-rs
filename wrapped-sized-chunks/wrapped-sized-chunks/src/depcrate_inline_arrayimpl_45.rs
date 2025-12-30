// Generated macro for impl_45 (impl)
macro_rules! Depcrate_inline_arrayimpl_45 {
() => {
// Module: crate::inline_array
// Provides: {"impl_45"}
// Dependencies: {}
impl < A , T > IntoIterator for InlineArray < A , T > { type Item = A ; type IntoIter = Iter < A , T > ; fn into_iter (self) -> Self :: IntoIter { Iter { array : self } } }
};
}
