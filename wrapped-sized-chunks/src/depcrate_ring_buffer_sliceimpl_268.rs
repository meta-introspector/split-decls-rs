// Generated macro for impl_268 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_268 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'a , A : Hash + 'a , const N : usize > Hash for Slice < 'a , A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
};
}
