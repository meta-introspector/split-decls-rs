// Generated macro for impl_236 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_236 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_236"}
// Dependencies: {}
impl < A , const N : usize > Iterator for OwnedIter < A , N > { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . buffer . pop_front () } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . buffer . len () , Some (self . buffer . len ())) } }
};
}
