// Generated macro for impl_279 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_279 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_279"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
