// Generated macro for impl_301 (impl)
macro_rules! Depcrate_ring_bufferimpl_301 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_301"}
// Dependencies: {}
impl < A , const N : usize > HasLength for RingBuffer < A , N > { # [doc = " Get the length of the ring buffer."] # [inline] # [must_use] fn len (& self) -> usize { self . length } }
};
}
