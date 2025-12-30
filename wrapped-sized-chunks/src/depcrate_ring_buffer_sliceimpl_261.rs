// Generated macro for impl_261 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_261 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < SliceMut < 'a , A , N > > for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & SliceMut < 'a , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
