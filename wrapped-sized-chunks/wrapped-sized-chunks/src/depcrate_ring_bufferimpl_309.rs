// Generated macro for impl_309 (impl)
macro_rules! Depcrate_ring_bufferimpl_309 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_309"}
// Dependencies: {}
impl < A : PartialEq , const N : usize > PartialEq for RingBuffer < A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
