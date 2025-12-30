// Generated macro for impl_159 (impl)
macro_rules! Depcrate_sparse_chunkimpl_159 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_159"}
// Dependencies: {}
impl < A , const N : usize > PartialEq for SparseChunk < A , N > where A : PartialEq , BitsImpl < N > : Bits , { fn eq (& self , other : & Self) -> bool { if self . map != other . map { return false ; } for index in self . indices () { if self . get (index) != other . get (index) { return false ; } } true } }
};
}
