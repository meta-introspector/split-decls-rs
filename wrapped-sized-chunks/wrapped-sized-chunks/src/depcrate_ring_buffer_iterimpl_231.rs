// Generated macro for impl_231 (impl)
macro_rules! Depcrate_ring_buffer_iterimpl_231 {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Iterator for Drain < 'a , A , N > { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . buffer . pop_front () } # [inline] # [must_use] fn size_hint (& self) -> (usize , Option < usize >) { (self . buffer . len () , Some (self . buffer . len ())) } }
};
}
