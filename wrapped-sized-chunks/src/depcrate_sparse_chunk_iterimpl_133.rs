// Generated macro for impl_133 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_133 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for Drain < A , N > where BitsImpl < N > : Bits , { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . chunk . pop () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . chunk . len () ; (len , Some (len)) } }
};
}
