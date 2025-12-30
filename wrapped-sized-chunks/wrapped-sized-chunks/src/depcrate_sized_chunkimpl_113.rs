// Generated macro for impl_113 (impl)
macro_rules! Depcrate_sized_chunkimpl_113 {
() => {
// Module: crate::sized_chunk
// Provides: {"impl_113"}
// Dependencies: {}
impl < A , const N : usize > IntoIterator for Chunk < A , N > { type Item = A ; type IntoIter = Iter < A , N > ; fn into_iter (self) -> Self :: IntoIter { Iter { chunk : self } } }
};
}
