// Generated macro for impl_210 (impl)
macro_rules! Depcrate_ring_buffer_indeximpl_210 {
() => {
// Module: crate::ring_buffer::index
// Provides: {"impl_210"}
// Dependencies: {}
impl < const N : usize > DoubleEndedIterator for IndexIter < N > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining > 0 { self . remaining -= 1 ; Some (self . right_index . dec ()) } else { None } } }
};
}
