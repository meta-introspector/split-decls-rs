// Generated macro for impl_221 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_221 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , A , const N : usize > DoubleEndedIterator for Iter < 'a , A , N > { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; Some (unsafe { & * self . buffer . ptr (self . right_index . dec ()) }) } } }
};
}
