// Generated macro for impl_262 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_262 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_262"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < RingBuffer < A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & RingBuffer < A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
