// Generated macro for impl_314 (impl)
macro_rules! Depcrate_ring_bufferimpl_314 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_314"}
// Dependencies: {}
impl < A : PartialOrd , const N : usize > PartialOrd for RingBuffer < A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
