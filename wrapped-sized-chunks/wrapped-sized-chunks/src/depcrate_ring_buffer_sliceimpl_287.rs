// Generated macro for impl_287 (impl)
macro_rules! Depcrate_ring_buffer_sliceimpl_287 {
() => {
// Module: crate::ring_buffer::slice
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'a , A : Hash + 'a , const N : usize > Hash for SliceMut < 'a , A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
};
}
