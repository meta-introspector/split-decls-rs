// Generated macro for SliceMut (struct)
macro_rules! Depcrate_ring_buffer_sliceSliceMut {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"SliceMut"}
// Dependencies: {}
# [doc = " An indexable representation of a mutable subset of a `RingBuffer`."] pub struct SliceMut < 'a , A , const N : usize > { pub (crate) buffer : & 'a mut RingBuffer < A , N > , pub (crate) range : Range < usize > , }
};
}
