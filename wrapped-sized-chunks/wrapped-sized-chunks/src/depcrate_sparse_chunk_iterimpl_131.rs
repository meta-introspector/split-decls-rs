// Generated macro for impl_131 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_131 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for IterMut < 'a , A , N > where BitsImpl < N > : Bits , { type Item = & 'a mut A ; fn next (& mut self) -> Option < Self :: Item > { if let Some (index) = self . bitmap . first_index () { self . bitmap . set (index , false) ; unsafe { let p : * mut A = & mut self . chunk . values_mut () [index] ; Some (& mut * p) } } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (SparseChunk :: < A , N > :: CAPACITY)) } }
};
}
