// Generated macro for impl_110 (impl)
macro_rules! Depcrate_sized_chunkimpl_110 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a , A , const N : usize > IntoIterator for & 'a mut Chunk < A , N > { type Item = & 'a mut A ; type IntoIter = SliceIterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
