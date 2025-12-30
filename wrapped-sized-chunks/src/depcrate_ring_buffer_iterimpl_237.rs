// Generated macro for impl_237 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_237 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_237"}
// Dependencies: {}
impl < A , const N : usize > DoubleEndedIterator for OwnedIter < A , N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . buffer . pop_back () } }
};
}
