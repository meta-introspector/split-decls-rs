// Generated macro for impl_156 (impl)
macro_rules! Depcrate_sparse_chunkimpl_156 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_156"}
// Dependencies: {}
impl < A , const N : usize > IndexMut < usize > for SparseChunk < A , N > where BitsImpl < N > : Bits , { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { self . get_mut (index) . unwrap () } }
};
}
