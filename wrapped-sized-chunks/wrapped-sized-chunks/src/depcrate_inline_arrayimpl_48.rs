// Generated macro for impl_48 (impl)
macro_rules! Depcrate_inline_arrayimpl_48 {
() => {
// Module: crate::inline_array
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , A , T > IntoIterator for & 'a mut InlineArray < A , T > { type Item = & 'a mut A ; type IntoIter = SliceIterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
