// Generated macro for impl_152 (impl)
macro_rules! Depcrate_sparse_chunkimpl_152 {
() => {
// Module: crate::sparse_chunk
// Provides: {"impl_152"}
// Dependencies: {}
impl < A : Clone , const N : usize > Clone for SparseChunk < A , N > where BitsImpl < N > : Bits , { fn clone (& self) -> Self { let mut out = Self :: new () ; for index in & self . map { out . insert (index , self [index] . clone ()) ; } out } }
};
}
