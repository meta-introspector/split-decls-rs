// Generated macro for impl_266 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_266 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_266"}
// Dependencies: {}
impl < 'a , A : Ord + 'a , const N : usize > Ord for Slice < 'a , A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
