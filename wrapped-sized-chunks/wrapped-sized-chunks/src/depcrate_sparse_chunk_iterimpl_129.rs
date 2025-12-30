// Generated macro for impl_129 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_129 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for Iter < 'a , A , N > where BitsImpl < N > : Bits , { type Item = & 'a A ; fn next (& mut self) -> Option < Self :: Item > { self . indices . next () . map (| index | & self . chunk . values () [index]) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (SparseChunk :: < A , N > :: CAPACITY)) } }
};
}
