// Generated macro for impl_315 (impl)
macro_rules! Depcrate_ring_bufferimpl_315 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_315"}
// Dependencies: {}
impl < A : Ord , const N : usize > Ord for RingBuffer < A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
