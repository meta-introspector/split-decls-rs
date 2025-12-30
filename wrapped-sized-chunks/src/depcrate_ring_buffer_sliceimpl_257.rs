// Generated macro for impl_257 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_257 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > From < & 'a RingBuffer < A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn from (buffer : & 'a RingBuffer < A , N >) -> Self { Slice { range : Range { start : 0 , end : buffer . len () , } , buffer , } } }
};
}
