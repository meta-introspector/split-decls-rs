// Generated macro for impl_157 (impl)
macro_rules! Depcrate_sparse_chunkimpl_157 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_157"}
// Dependencies: {}
impl < A , const N : usize > IntoIterator for SparseChunk < A , N > where BitsImpl < N > : Bits , { type Item = A ; type IntoIter = Drain < A , N > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . drain () } }
};
}
