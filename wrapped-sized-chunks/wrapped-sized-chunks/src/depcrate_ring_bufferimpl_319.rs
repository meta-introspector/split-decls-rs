// Generated macro for impl_319 (impl)
macro_rules! Depcrate_ring_bufferimpl_319 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_319"}
// Dependencies: {}
impl < A : Hash , const N : usize > Hash for RingBuffer < A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
};
}
