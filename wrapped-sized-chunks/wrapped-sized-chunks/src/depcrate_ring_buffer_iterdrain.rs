// Generated macro for Drain (struct)
macro_rules! Depcrate_ring_buffer_iterDrain {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over a `RingBuffer`."] pub struct Drain < 'a , A , const N : usize > { pub (crate) buffer : & 'a mut RingBuffer < A , N > , }
};
}
