// Generated macro for impl_260 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_260 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_260"}
// Dependencies: {}
impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq for Slice < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
