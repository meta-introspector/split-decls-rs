// Generated macro for impl_275 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_275 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > From < & 'a mut RingBuffer < A , N > > for SliceMut < 'a , A , N > { # [must_use] fn from (buffer : & 'a mut RingBuffer < A , N >) -> Self { SliceMut { range : Range { start : 0 , end : buffer . len () , } , buffer , } } }
};
}
