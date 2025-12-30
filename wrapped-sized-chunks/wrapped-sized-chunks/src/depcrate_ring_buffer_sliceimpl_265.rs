// Generated macro for impl_265 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_265 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'a , A : PartialOrd + 'a , const N : usize > PartialOrd for Slice < 'a , A , N > { # [inline] # [must_use] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
