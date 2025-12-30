// Generated macro for impl_227 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_227 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'a , A , const N : usize > DoubleEndedIterator for IterMut < 'a , A , N > where A : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { if self . remaining == 0 { None } else { self . remaining -= 1 ; let index = self . right_index . dec () ; Some (unsafe { & mut * self . mut_ptr (index) }) } } }
};
}
