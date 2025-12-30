// Generated macro for impl_68 (impl)
macro_rules! Depcrate_sized_chunk_iterimpl_68 {
() => {
// Module: crate::sized_chunk::iter
// Provides: {"impl_68"}
// Dependencies: {}
impl < A , const N : usize > Iterator for Iter < A , N > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_front ()) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . chunk . len () , Some (self . chunk . len ())) } }
};
}
