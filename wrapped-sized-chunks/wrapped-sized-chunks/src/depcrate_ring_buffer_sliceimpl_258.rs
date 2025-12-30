// Generated macro for impl_258 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_258 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a , A : 'a , const N : usize > Clone for Slice < 'a , A , N > { # [inline] # [must_use] fn clone (& self) -> Self { Slice { buffer : self . buffer , range : self . range . clone () , } } }
};
}
