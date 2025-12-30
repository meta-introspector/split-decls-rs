// Generated macro for impl_271 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_271 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > HasLength for SliceMut < 'a , A , N > { # [doc = " Get the length of the slice."] # [inline] # [must_use] fn len (& self) -> usize { self . range . end - self . range . start } }
};
}
