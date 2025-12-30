// Generated macro for impl_281 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_281 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < RingBuffer < A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & RingBuffer < A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
