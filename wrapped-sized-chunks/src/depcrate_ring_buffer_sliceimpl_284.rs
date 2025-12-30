// Generated macro for impl_284 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_284 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'a , A : PartialOrd + 'a , const N : usize > PartialOrd for SliceMut < 'a , A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
