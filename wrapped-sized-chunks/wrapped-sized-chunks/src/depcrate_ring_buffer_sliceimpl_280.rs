// Generated macro for impl_280 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_280 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_280"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < Slice < 'a , A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Slice < 'a , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
