// Generated macro for impl_317 (impl)
macro_rules! Depcrate_ring_bufferimpl_317 {
() => {
// Module: crate::ring_buffer
// Provides: {"impl_317"}
// Dependencies: {}
impl < 'a , A : Clone + 'a , const N : usize > Extend < & 'a A > for RingBuffer < A , N > { # [inline] fn extend < I : IntoIterator < Item = & 'a A > > (& mut self , iter : I) { for item in iter { self . push_back (item . clone ()) ; } } }
};
}
