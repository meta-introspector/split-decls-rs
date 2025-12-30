// Generated macro for impl_316 (impl)
macro_rules! Depcrate_ring_bufferimpl_316 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_316"}
// Dependencies: {}
impl < A , const N : usize > Extend < A > for RingBuffer < A , N > { # [inline] fn extend < I : IntoIterator < Item = A > > (& mut self , iter : I) { for item in iter { self . push_back (item) ; } } }
};
}
