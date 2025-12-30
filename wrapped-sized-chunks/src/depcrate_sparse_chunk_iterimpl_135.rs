// Generated macro for impl_135 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_135 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_135"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for OptionIter < 'a , A , N > where BitsImpl < N > : Bits , { type Item = Option < & 'a A > ; fn next (& mut self) -> Option < Self :: Item > { if self . index < N { let result = self . chunk . get (self . index) ; self . index += 1 ; Some (result) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (SparseChunk :: < A , N > :: CAPACITY - self . index , Some (SparseChunk :: < A , N > :: CAPACITY - self . index) ,) } }
};
}
