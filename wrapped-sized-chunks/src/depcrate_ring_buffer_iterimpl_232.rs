// Generated macro for impl_232 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_232 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > DoubleEndedIterator for Drain < 'a , A , N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { self . buffer . pop_back () } }
};
}
