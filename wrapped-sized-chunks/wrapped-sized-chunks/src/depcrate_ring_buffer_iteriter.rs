// Generated macro for Iter (struct)
macro_rules! Depcrate_ring_buffer_iterIter {
() => {
// Module: crate::ring_buffer::iter
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " A reference iterator over a `RingBuffer`."] pub struct Iter < 'a , A , const N : usize > { pub (crate) buffer : & 'a RingBuffer < A , N > , pub (crate) left_index : RawIndex < N > , pub (crate) right_index : RawIndex < N > , pub (crate) remaining : usize , }
};
}
