// Generated macro for impl_322 (impl)
macro_rules! Depcrate_ring_bufferimpl_322 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_322"}
// Dependencies: {}
impl < A , const N : usize > FromIterator < A > for RingBuffer < A , N > { # [must_use] fn from_iter < I : IntoIterator < Item = A > > (iter : I) -> Self { let mut buffer = RingBuffer :: new () ; buffer . extend (iter) ; buffer } }
};
}
