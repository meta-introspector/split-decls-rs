// Generated macro for impl_137 (impl)
macro_rules! Depcrate_sparse_chunk_iterimpl_137 {
() => {
// Module: crate::sparse_chunk::iter
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'a , A , const N : usize > Iterator for OptionIterMut < 'a , A , N > where BitsImpl < N > : Bits , { type Item = Option < & 'a mut A > ; fn next (& mut self) -> Option < Self :: Item > { if self . index < N { let result = if self . chunk . map . get (self . index) { unsafe { let p : * mut A = & mut self . chunk . values_mut () [self . index] ; Some (Some (& mut * p)) } } else { Some (None) } ; self . index += 1 ; result } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (SparseChunk :: < A , N > :: CAPACITY - self . index , Some (SparseChunk :: < A , N > :: CAPACITY - self . index) ,) } }
};
}
