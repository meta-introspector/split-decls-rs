// Generated macro for impl_74 (impl)
macro_rules! Depcrate_sized_chunk_iterimpl_74 {
() => {
// Module: crate::sized_chunk::iter
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a , A , const N : usize > DoubleEndedIterator for Drain < 'a , A , N > where A : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { if self . chunk . is_empty () { None } else { Some (self . chunk . pop_back ()) } } }
};
}
