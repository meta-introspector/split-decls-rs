// Generated macro for impl_69 (impl)
macro_rules! Depcrate_sized_chunk_iterimpl_69 {
() => {
// Module: crate::sized_chunk::iter
// Provides: {"impl_69"}
// Dependencies: {}
impl < A , const N : usize > DoubleEndedIterator for Iter < A , N > { fn next_back (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_back ()) } } }
};
}
