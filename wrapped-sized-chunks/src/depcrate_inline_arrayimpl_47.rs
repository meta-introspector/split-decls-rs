// Generated macro for impl_47 (impl)
macro_rules! Depcrate_inline_arrayimpl_47 {
() => {
// Module: crate::inline_array
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , A , T > IntoIterator for & 'a InlineArray < A , T > { type Item = & 'a A ; type IntoIter = SliceIter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
