// Generated macro for impl_109 (impl)
macro_rules! Depcrate_sized_chunkimpl_109 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , A , const N : usize > IntoIterator for & 'a Chunk < A , N > { type Item = & 'a A ; type IntoIter = SliceIter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
