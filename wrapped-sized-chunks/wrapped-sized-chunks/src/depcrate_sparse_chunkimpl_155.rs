// Generated macro for impl_155 (impl)
macro_rules! Depcrate_sparse_chunkimpl_155 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_155"}
// Dependencies: {}
impl < A , const N : usize > Index < usize > for SparseChunk < A , N > where BitsImpl < N > : Bits , { type Output = A ; # [inline] fn index (& self , index : usize) -> & Self :: Output { self . get (index) . unwrap () } }
};
}
