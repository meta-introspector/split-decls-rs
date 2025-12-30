// Generated macro for Slice (struct)
macro_rules! Depcrate_ring_buffer_sliceSlice {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"Slice"}
// Dependencies: {}
# [doc = " An indexable representation of a subset of a `RingBuffer`."] pub struct Slice < 'a , A , const N : usize > { pub (crate) buffer : & 'a RingBuffer < A , N > , pub (crate) range : Range < usize > , }
};
}
