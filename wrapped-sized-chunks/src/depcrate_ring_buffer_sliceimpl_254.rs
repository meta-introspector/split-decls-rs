// Generated macro for impl_254 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_254 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > HasLength for Slice < 'a , A , N > { # [doc = " Get the length of the slice."] # [inline] # [must_use] fn len (& self) -> usize { self . range . end - self . range . start } }
};
}
