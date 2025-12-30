// Generated macro for impl_285 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_285 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'a , A : Ord + 'a , const N : usize > Ord for SliceMut < 'a , A , N > { # [inline] # [must_use] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
