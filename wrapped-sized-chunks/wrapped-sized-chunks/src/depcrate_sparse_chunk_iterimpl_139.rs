// Generated macro for impl_139 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_139 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for OptionDrain < A , N > where BitsImpl < N > : Bits , { type Item = Option < A > ; fn next (& mut self) -> Option < Self :: Item > { if self . index < N { let result = self . chunk . remove (self . index) ; self . index += 1 ; Some (result) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (SparseChunk :: < A , N > :: CAPACITY - self . index , Some (SparseChunk :: < A , N > :: CAPACITY - self . index) ,) } }
};
}
